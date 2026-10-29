#!/usr/bin/env python3
"""Deny agent edits that grow the visible surface of types listed in locked-classes.txt."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LOCK_FILE = Path(__file__).resolve().parent / "locked-classes.txt"
CONSTANT_FILES = (ROOT / "quick_starts" / "run.mjs",)
EDIT_TOOLS = {"Write", "StrReplace", "Delete"}

KINDS = ("class", "record", "struct", "interface", "enum", "trait")


def main() -> None:
    if "--show" in sys.argv:
        show()
        return
    raw = sys.stdin.read()
    if not raw.strip():
        emit(allow())
        return
    try:
        data = json.loads(raw)
    except json.JSONDecodeError:
        emit(deny("locked type", "The edit could not be checked."))
        return
    if not isinstance(data, dict):
        emit(deny("locked type", "The edit could not be checked."))
        return
    emit(decide(data))


def decide(data: dict) -> dict:
    if "command" in data and "tool_name" not in data:
        return check_shell(str(data.get("command") or ""))
    tool = str(data.get("tool_name") or "")
    tool_input = data.get("tool_input") or {}
    if isinstance(tool_input, str):
        try:
            tool_input = json.loads(tool_input)
        except json.JSONDecodeError:
            return deny("locked type", "The edit could not be checked.")
    if not isinstance(tool_input, dict):
        return deny("locked type", "The edit could not be checked.")
    if tool in {"Shell", "shell"}:
        return check_shell(str(tool_input.get("command") or ""))
    if tool not in EDIT_TOOLS:
        return allow()
    cwd = str(data.get("cwd") or ROOT)
    path = tool_path(tool_input, cwd)
    if path is None:
        return deny("locked type", "The edit could not be checked.")
    if path.resolve() == LOCK_FILE.resolve():
        return deny_list()
    if path.resolve() in {file.resolve() for file in CONSTANT_FILES}:
        return deny_constant(path)
    owners = owners_of(path)
    if not owners:
        return allow()
    if tool == "Delete":
        return deny(owners[0])
    proposed = proposed_text(tool, tool_input, path)
    if proposed is None:
        return deny(owners[0])
    for owner in owners:
        result = compare(owner, path, proposed)
        if result["permission"] != "allow":
            return result
    return allow()


def show() -> None:
    names = locked_names()
    if not names:
        print("No types are locked.")
        return
    for name in names:
        print(name)
        files = files_for(name)
        if not files:
            print("  (no library file found)")
            continue
        for path in files:
            print(f"  {path.relative_to(ROOT).as_posix()}")
            surface = public_surface(path.read_text(), lang_of(path), name)
            if surface is None:
                print("    (public surface could not be read)")
                continue
            for sig in surface.values():
                print(f"    {sig}")
        print()


def compare(name: str, path: Path, proposed: str) -> dict:
    current_text = path.read_text() if path.exists() else ""
    lang = lang_of(path)
    current = public_surface(current_text, lang, name) if current_text else {}
    if current_text and current is None:
        return deny(name)
    after = public_surface(proposed, lang, name)
    if after is None:
        return deny(name)
    before = current or {}
    after = after or {}
    if before == after:
        return allow()
    added = [after[key] for key in after.keys() - before.keys()]
    removed = [before[key] for key in before.keys() - after.keys()]
    changed = [
        f"{before[key]} -> {after[key]}"
        for key in before.keys() & after.keys()
        if before[key] != after[key]
    ]
    parts = []
    if added:
        parts.append("added: " + "; ".join(added[:4]))
    if removed:
        parts.append("removed: " + "; ".join(removed[:4]))
    if changed:
        parts.append("changed: " + "; ".join(changed[:4]))
    return deny(name, "Rejected: " + " | ".join(parts) if parts else "Rejected: surface changed")


def proposed_text(tool: str, tool_input: dict, path: Path) -> str | None:
    if tool == "Write":
        contents = tool_input.get("contents", tool_input.get("content"))
        return None if not isinstance(contents, str) else contents
    if tool != "StrReplace":
        return None
    old = tool_input.get("old_string", tool_input.get("old_str"))
    new = tool_input.get("new_string", tool_input.get("new_str"))
    if not isinstance(old, str) or not isinstance(new, str) or old == "":
        return None
    if not path.exists():
        return None
    text = path.read_text()
    if old not in text:
        return None
    count = -1 if tool_input.get("replace_all") else 1
    return text.replace(old, new, count)


def check_shell(command: str) -> dict:
    if writes(command, [LOCK_FILE]):
        return deny_list()
    if writes(command, list(CONSTANT_FILES)):
        return deny_constant(CONSTANT_FILES[0])
    for name in locked_names():
        for path in files_for(name):
            if not writes(command, [path]):
                continue
            proposed = shell_proposed_text(command, path)
            if proposed is None:
                return deny(
                    name,
                    "Shell cannot verify the public surface. "
                    "Use Write or StrReplace; body and private edits are allowed; signatures are not.",
                )
            result = compare(name, path, proposed)
            if result["permission"] != "allow":
                return result
    return allow()


def shell_proposed_text(command: str, path: Path) -> str | None:
    if unverifiable_shell_write(command):
        return None
    tokens = mention_tokens(path)
    recovered = redirect_content(command, tokens)
    if recovered is None:
        return None
    op, content = recovered
    if op == ">":
        return content
    current = path.read_text() if path.exists() else ""
    return current + content


def unverifiable_shell_write(command: str) -> bool:
    if re.search(r"\btee\b", command):
        return True
    if re.search(r"\bsed\b", command) and re.search(r"(^|\s)--in-place|(^|\s)-[A-Za-z]*i\b", command):
        return True
    if re.search(r"\bperl\b", command) and re.search(r"(^|\s)-[A-Za-z]*i", command):
        return True
    if re.search(r"\b(python3?|node|ruby)\b", command) and re.search(
        r"write_text|write_bytes|\.write\b|truncate\s*\(|['\"][wax]\+?['\"]",
        command,
    ):
        return True
    if re.search(r"\brm\b", command):
        return True
    if re.search(r"\bgit\s+(checkout|restore)\b", command):
        return True
    return False


def writes(command: str, paths: list[Path]) -> bool:
    tokens = [token for path in paths for token in mention_tokens(path)]
    if not tokens or not any(token in command for token in tokens):
        return False
    if redirect_targets(command, tokens):
        return True
    return unverifiable_shell_write(command)


def redirect_targets(command: str, tokens: list[str]) -> bool:
    for match in re.finditer(r"(?:^|[^>\d&])>{1,2}\s*([^\s;&|]+)", command):
        target = match.group(1).strip("'\"")
        if any(token in target for token in tokens):
            return True
    return False


def redirect_content(command: str, tokens: list[str]) -> tuple[str, str] | None:
    for match in re.finditer(r"(?:^|[^>\d&])(>{1,2})\s*([^\s;&|]+)", command):
        target = match.group(2).strip("'\"")
        if not any(token in target for token in tokens):
            continue
        op = match.group(1)
        heredoc = re.search(r"<<\s*['\"]?(\w+)['\"]?", command)
        if heredoc:
            tag = heredoc.group(1)
            after = command[heredoc.end() :]
            if after.startswith("\n"):
                after = after[1:]
            end = re.search(rf"(?m)^{re.escape(tag)}\s*$", after)
            if end is None:
                return None
            return op, after[: end.start()]
        prefix = command[: match.start()].rstrip()
        quoted = re.search(r"""(?:echo|printf)\s+(?:-e\s+)?(['"])(.*)(\1)\s*$""", prefix, re.DOTALL)
        if quoted:
            body = quoted.group(2)
            if re.search(r"\becho\b", prefix) and not body.endswith("\n"):
                body += "\n"
            return op, body
        return None
    return None


def mention_tokens(path: Path) -> list[str]:
    try:
        relative = path.resolve().relative_to(ROOT).as_posix()
    except ValueError:
        relative = path.as_posix()
    return [token for token in (relative, path.name, str(path.resolve())) if token]


def tool_path(tool_input: dict, cwd: str) -> Path | None:
    raw = next((tool_input[key] for key in ("path", "file_path") if tool_input.get(key)), None)
    if not isinstance(raw, str) or not raw:
        return None
    path = Path(raw)
    if path.is_absolute():
        return path
    return Path(cwd) / path


def owners_of(path: Path) -> list[str]:
    owner = owner_of(path)
    return [] if owner is None else [owner]


def owner_of(path: Path) -> str | None:
    resolved = path.resolve()
    for name in locked_names():
        if any(resolved == candidate.resolve() for candidate in files_for(name)):
            return name
    return None


def locked_names() -> list[str]:
    if not LOCK_FILE.exists():
        return []
    names = []
    for line in LOCK_FILE.read_text().splitlines():
        name = line.split("#", 1)[0].strip()
        if not name or name.startswith("--"):
            continue
        names.append(name)
    return names


def files_for(name: str) -> list[Path]:
    snake = re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()
    roots = (
        (ROOT / "csharp" / "Datasources", f"{name}.cs", ".cs"),
        (ROOT / "typescript" / "datasources" / "src", f"{name}.ts", ".ts"),
        (ROOT / "rust" / "datasources" / "src", f"{snake}.rs", ".rs"),
    )
    found = []
    for root, pattern, suffix in roots:
        if not root.is_dir():
            continue
        matches = [path for path in sorted(root.rglob(pattern)) if not excluded(path)]
        found.extend(matches or declared_in(root, suffix, name))
    return found


def declared_in(root: Path, suffix: str, name: str) -> list[Path]:
    found = []
    needle = re.compile(rf"\b{re.escape(name)}\b")
    for path in sorted(root.rglob(f"*{suffix}")):
        if excluded(path):
            continue
        text = path.read_text()
        if needle.search(text) and public_surface(text, lang_of(path), name) is not None:
            found.append(path)
    return found


def excluded(path: Path) -> bool:
    if path.name.endswith(".test.ts"):
        return True
    return any(part in {"tests", "test", "samples"} or part.endswith(".Tests") for part in path.parts)


def lang_of(path: Path) -> str:
    return {".cs": "cs", ".ts": "ts", ".rs": "rs"}.get(path.suffix, "")


def public_surface(text: str, lang: str, name: str) -> dict[str, str] | None:
    if lang == "cs":
        return cs_surface(text, name)
    if lang == "ts":
        return ts_surface(text, name)
    if lang == "rs":
        return rs_surface(text, name)
    return None


def cs_surface(text: str, name: str) -> dict[str, str] | None:
    found = find_decls(text, "cs", name, ("class", "record", "struct", "interface", "enum"))
    if not found:
        return None
    sigs: dict[str, str] = {}
    for kind, header, body_open, body_close in found:
        add_type(sigs, header, kind, name)
        walk_members(text, body_open, body_close, "cs", sigs, kind == "interface")
    return sigs


def ts_surface(text: str, name: str) -> dict[str, str] | None:
    found = find_decls(text, "ts", name, ("class", "interface"))
    if not found:
        return None
    sigs: dict[str, str] = {}
    for kind, header, body_open, body_close in found:
        add_type(sigs, header, kind, name)
        walk_members(text, body_open, body_close, "ts", sigs, kind == "interface")
    return sigs


def rs_surface(text: str, name: str) -> dict[str, str] | None:
    sigs: dict[str, str] = {}
    found = False
    for kind, header, body_open, body_close in find_decls(text, "rs", name, ("struct", "enum", "trait")):
        found = True
        add_type(sigs, header, kind, name)
        walk_members(text, body_open, body_close, "rs", sigs, kind == "trait")
    for header, body_open, body_close, trait in rust_impls(text, name):
        found = True
        if trait:
            add(sigs, "1", trait)
        walk_members(text, body_open, body_close, "rs", sigs, bool(trait))
    return sigs if found else None


def add_type(sigs: dict[str, str], header: str, kind: str, name: str) -> None:
    generics = generics_after(header, name)
    prefix = "export " if re.search(r"\bexport\b", header) else ""
    if re.search(r"\bpublic\b", header):
        prefix = "public "
    elif re.search(r"\bpub\b", header) and "pub(" not in header:
        prefix = "pub "
    add(sigs, "0", f"{prefix}{kind} {name}{generics}")
    for base in bases_of(header):
        add(sigs, "1", base)


def walk_members(
    text: str,
    start: int,
    end: int,
    lang: str,
    sigs: dict[str, str],
    implicit_public: bool,
) -> None:
    i = start + 1
    depth = 1
    boundary = True
    while i < end:
        noise = consume_noise(text, i, lang)
        if noise is not None:
            i = noise
            continue
        if text.startswith("#[", i) or text.startswith("#!", i):
            i = skip_brackets(text, i + 1, lang)
            continue
        char = text[i]
        if char == "{":
            depth += 1
            boundary = False
            i += 1
            continue
        if char == "}":
            depth -= 1
            boundary = depth == 1
            i += 1
            continue
        if char == ";" and depth == 1:
            boundary = True
            i += 1
            continue
        if char == "," and lang == "rs" and depth == 1:
            boundary = True
            i += 1
            continue
        if char == "#" and lang == "cs" and depth == 1:
            newline = text.find("\n", i)
            i = end if newline < 0 else newline + 1
            continue
        if depth == 1 and boundary and not char.isspace():
            start_at = member_start(text, i, end, lang, implicit_public)
            if start_at is None:
                boundary = False
            else:
                signature, i = read_signature(text, start_at, end, lang)
                add(sigs, "2", signature)
                boundary = False
                continue
        i += 1


CS_LOCKED = {"public", "protected", "internal"}
CS_MODIFIERS = {
    "abstract", "async", "const", "event", "explicit", "extern", "fixed",
    "implicit", "new", "override", "partial", "readonly", "ref", "required",
    "scoped", "sealed", "static", "unsafe", "virtual", "volatile",
}


def member_start(text: str, index: int, limit: int, lang: str, implicit_public: bool) -> int | None:
    if lang == "ts":
        return None if hidden_ts(text, index) else index
    if lang == "rs":
        return index if match_word(text, index, "pub") or implicit_public else None
    cursor = skip_attributes(text, index, limit, lang)
    locked, private = leading_access(text, cursor, limit)
    if locked or (implicit_public and not private):
        return cursor
    return None


def leading_access(text: str, index: int, limit: int) -> tuple[bool, bool]:
    locked = False
    private = False
    while index < limit:
        noise = consume_noise(text, index, "cs")
        if noise is not None:
            index = noise
            continue
        if text[index].isspace():
            index += 1
            continue
        word = word_at(text, index)
        if word in CS_LOCKED:
            locked = True
        elif word == "private":
            private = True
        elif word not in CS_MODIFIERS:
            break
        index += len(word)
    return locked, private


def word_at(text: str, index: int) -> str:
    end = index
    while end < len(text) and (text[end].isalnum() or text[end] == "_"):
        end += 1
    return text[index:end]


def hidden_ts(text: str, index: int) -> bool:
    if text[index] == "#":
        return True
    return match_word(text, index, "private")


def skip_attributes(text: str, index: int, limit: int, lang: str) -> int:
    while index < limit:
        noise = consume_noise(text, index, lang)
        if noise is not None:
            index = noise
            continue
        if text[index].isspace():
            index += 1
            continue
        if text[index] == "[":
            index = skip_brackets(text, index, lang)
            continue
        break
    return index


def skip_brackets(text: str, index: int, lang: str) -> int:
    depth = 0
    while index < len(text):
        noise = consume_noise(text, index, lang)
        if noise is not None:
            index = noise
            continue
        if text[index] == "[":
            depth += 1
        elif text[index] == "]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    return index


def read_signature(text: str, start: int, limit: int, lang: str) -> tuple[str, int]:
    index = start
    paren = angle = 0
    seen_paren = False
    while index < limit:
        noise = consume_noise(text, index, lang)
        if noise is not None:
            index = noise
            continue
        if text.startswith("=>", index) and paren == 0 and angle == 0 and not seen_paren:
            return text[start:index], index
        skipped = skip_pub_restriction(text, index, limit, lang)
        if skipped is not None:
            index = skipped
            continue
        char = text[index]
        if char == "<":
            angle += 1
        elif char == ">" and angle > 0:
            angle -= 1
        elif char == "(":
            paren += 1
            seen_paren = True
        elif char == ")":
            if paren:
                paren -= 1
            if seen_paren and paren == 0 and angle == 0:
                return text[start : index + 1], index + 1
        elif paren == 0 and angle == 0 and (char in "{;" or (lang == "rs" and char == ",")):
            return text[start:index], index
        index += 1
    return text[start:index], index


def skip_pub_restriction(text: str, index: int, limit: int, lang: str) -> int | None:
    if lang != "rs" or not match_word(text, index, "pub"):
        return None
    cursor = index + 3
    while cursor < limit and text[cursor].isspace():
        cursor += 1
    if cursor >= limit or text[cursor] != "(":
        return None
    depth = 0
    while cursor < limit:
        noise = consume_noise(text, cursor, lang)
        if noise is not None:
            cursor = noise
            continue
        if text[cursor] == "(":
            depth += 1
        elif text[cursor] == ")":
            depth -= 1
            if depth == 0:
                return cursor + 1
        cursor += 1
    return limit


def find_decls(text: str, lang: str, name: str, kinds: tuple[str, ...]) -> list[tuple[str, str, int, int]]:
    found = []
    for kind, index in scan_words(text, lang, kinds):
        if not decl_name_is(text, index + len(kind), name):
            continue
        grabbed = grab_block(text, index, lang)
        if grabbed is None:
            continue
        header, open_brace, close_brace = grabbed
        found.append((kind, header, open_brace, close_brace))
    return found


def rust_impls(text: str, name: str) -> list[tuple[str, int, int, str]]:
    found = []
    for _, index in scan_words(text, "rs", ("impl",)):
        grabbed = grab_block(text, index, "rs")
        if grabbed is None:
            continue
        header, open_brace, close_brace = grabbed
        trait = impl_trait(header, name)
        if trait is None:
            continue
        found.append((header, open_brace, close_brace, trait))
    return found


def impl_trait(header: str, name: str) -> str | None:
    body = re.sub(r"^impl\b", "", header).strip()
    body = strip_leading_generic(body)
    body = re.split(r"\bwhere\b", body, maxsplit=1)[0].strip()
    if re.search(r"\bfor\b", body):
        trait, target = re.split(r"\bfor\b", body, maxsplit=1)
        if type_name(target) != name:
            return None
        return " ".join(trait.split())
    return "" if type_name(body) == name else None


def decl_name_is(text: str, index: int, name: str) -> bool:
    while index < len(text) and text[index].isspace():
        index += 1
    return match_word(text, index, name)


def grab_block(text: str, start: int, lang: str) -> tuple[str, int, int] | None:
    index = start
    angle = 0
    while index < len(text):
        noise = consume_noise(text, index, lang)
        if noise is not None:
            index = noise
            continue
        char = text[index]
        if char == "<":
            angle += 1
        elif char == ">" and angle > 0:
            angle -= 1
        elif char == "{" and angle == 0:
            close = matching_brace(text, index, lang)
            if close is None:
                return None
            return text[start:index], index, close
        elif char == ";" and angle == 0:
            return None
        index += 1
    return None


def matching_brace(text: str, open_index: int, lang: str) -> int | None:
    depth = 0
    index = open_index
    while index < len(text):
        noise = consume_noise(text, index, lang)
        if noise is not None:
            index = noise
            continue
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    return None


def scan_words(text: str, lang: str, words: tuple[str, ...]) -> list[tuple[str, int]]:
    found = []
    index = 0
    depth = 0
    while index < len(text):
        noise = consume_noise(text, index, lang)
        if noise is not None:
            index = noise
            continue
        char = text[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth = max(0, depth - 1)
        elif depth == 0:
            for word in words:
                if match_word(text, index, word):
                    found.append((word, index))
                    break
        index += 1
    return found


def bases_of(header: str) -> list[str]:
    header = re.split(r"\bwhere\b", header, maxsplit=1)[0]
    chunks = []
    for keyword in ("implements", "extends"):
        if re.search(rf"\b{keyword}\b", header):
            chunks.append(re.split(rf"\b{keyword}\b", header, maxsplit=1)[1])
    if ":" in header and not chunks:
        chunks.append(header.split(":", 1)[1])
    bases = []
    for chunk in chunks:
        chunk = re.split(r"\b(implements|extends|where)\b", chunk, maxsplit=1)[0]
        bases.extend(part for part in split_csv(chunk) if part)
    return bases


def generics_after(header: str, name: str) -> str:
    match = re.search(rf"\b{re.escape(name)}\b", header)
    if match is None:
        return ""
    rest = strip_leading_generic(header[match.end() :])
    generic = header[match.end() : len(header) - len(rest)]
    return " ".join(generic.split())


def strip_leading_generic(text: str) -> str:
    trimmed = text.lstrip()
    if not trimmed.startswith("<"):
        return trimmed
    depth = 0
    for index, char in enumerate(trimmed):
        if char == "<":
            depth += 1
        elif char == ">":
            depth -= 1
            if depth == 0:
                return trimmed[index + 1 :].lstrip()
    return trimmed


def split_csv(text: str) -> list[str]:
    parts: list[str] = []
    buf: list[str] = []
    angle = paren = 0
    for char in text:
        if char == "<":
            angle += 1
        elif char == ">":
            angle = max(0, angle - 1)
        elif char == "(":
            paren += 1
        elif char == ")":
            paren = max(0, paren - 1)
        elif char == "," and angle == 0 and paren == 0:
            parts.append("".join(buf).strip())
            buf = []
            continue
        buf.append(char)
    tail = "".join(buf).strip()
    if tail:
        parts.append(tail)
    return parts


def type_name(fragment: str) -> str:
    fragment = " ".join(fragment.split())
    fragment = re.sub(r"^pub(\s*\([^)]*\))?\s+", "", fragment)
    fragment = re.sub(r"^(struct|enum|class|trait|interface|record)\s+", "", fragment)
    match = re.match(r"[A-Za-z_][A-Za-z0-9_]*", fragment)
    return match.group(0) if match else ""


def add(sigs: dict[str, str], kind: str, signature: str) -> None:
    shown = " ".join(signature.split()).strip()
    shown = re.sub(r"\s+,", ",", shown)
    shown = re.sub(r",\s*", ", ", shown)
    shown = re.sub(r"\(\s+", "(", shown)
    shown = re.sub(r"\s+\)", ")", shown)
    shown = re.sub(r",\s*\)", ")", shown)
    if not shown:
        return
    sigs[f"{kind}:{signature_key(shown)}"] = shown


def signature_key(text: str) -> str:
    return re.sub(r"\s*([(),<>?:|&.=+])\s*", r"\1", text).strip()


def match_word(text: str, index: int, word: str) -> bool:
    if not text.startswith(word, index):
        return False
    before = text[index - 1] if index else ""
    after_index = index + len(word)
    after = text[after_index] if after_index < len(text) else ""
    return not (before.isalnum() or before == "_") and not (after.isalnum() or after == "_")


def consume_noise(text: str, index: int, lang: str) -> int | None:
    if text.startswith("//", index):
        newline = text.find("\n", index)
        return len(text) if newline < 0 else newline + 1
    if text.startswith("/*", index):
        close = text.find("*/", index + 2)
        return len(text) if close < 0 else close + 2
    if lang == "rs":
        return rust_string(text, index)
    if text.startswith('@"', index) or text.startswith('$@"', index) or text.startswith('$"', index):
        return csharp_string(text, index)
    if index < len(text) and text[index] in "\"'`":
        return plain_string(text, index, text[index])
    return None


def rust_string(text: str, index: int) -> int | None:
    if text.startswith('r#"', index) or text.startswith('r##"', index):
        hashes = 0
        cursor = index + 1
        while cursor < len(text) and text[cursor] == "#":
            hashes += 1
            cursor += 1
        if cursor < len(text) and text[cursor] == '"':
            closer = '"' + ("#" * hashes)
            close = text.find(closer, cursor + 1)
            return len(text) if close < 0 else close + len(closer)
    if index < len(text) and text[index] == '"':
        return plain_string(text, index, '"')
    if index < len(text) and text[index] == "'":
        nxt = text[index + 1] if index + 1 < len(text) else ""
        nxt2 = text[index + 2] if index + 2 < len(text) else ""
        if nxt == "\\" and index + 3 < len(text) and text[index + 3] == "'":
            return index + 4
        if nxt2 == "'" and nxt != "":
            return index + 3
    return None


def csharp_string(text: str, index: int) -> int:
    cursor = index
    if text.startswith('$@"') or text.startswith('@"'):
        cursor = text.find('"', index)
    elif text.startswith('$"'):
        cursor = index + 1
    return plain_string(text, cursor, '"')


def plain_string(text: str, index: int, quote: str) -> int:
    cursor = index + 1
    while cursor < len(text):
        char = text[cursor]
        if char == "\\":
            cursor += 2
            continue
        if char == quote:
            return cursor + 1
        if char == "\n" and quote != "`":
            return cursor
        cursor += 1
    return cursor


def allow() -> dict:
    return {"permission": "allow"}


def deny(name: str, extra: str = "") -> dict:
    message = (
        f"{name} is listed in .cursor/hooks/locked-classes.txt. "
        "Do not add, remove, or change a public, internal, or protected member, "
        "a pub(crate) or pub(super) member, or a parameter. "
        "Do not subclass this type to reach a new member. "
        "Stop and ask the user how to proceed. Dependency injection is one design they may want. "
        "Body and private edits are allowed (prefer Write or StrReplace so the surface can be checked). "
        "The user removes the name from that file to change this API."
    )
    if extra:
        message = f"{message} {extra}"
    return {
        "permission": "deny",
        "user_message": f"{name} is locked.",
        "agent_message": message,
    }


def deny_constant(path: Path) -> dict:
    relative = path.resolve().relative_to(ROOT).as_posix()
    message = (
        f"{relative} is constant. Do not edit the e2e runner. "
        "Fix the published consumer packages, generate, or samples. "
        "E2E is npx @deterministic-code/datasources-cli in a consumer project."
    )
    return {
        "permission": "deny",
        "user_message": f"{relative} is constant.",
        "agent_message": message,
    }


def deny_list() -> dict:
    return {
        "permission": "deny",
        "user_message": "locked-classes.txt is locked.",
        "agent_message": (
            "Do not edit .cursor/hooks/locked-classes.txt. "
            "The user maintains that list."
        ),
    }


def emit(result: dict) -> None:
    sys.stdout.write(json.dumps(result) + "\n")


if __name__ == "__main__":
    main()
