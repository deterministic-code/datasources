#!/usr/bin/env python3
"""Generate Plan 1 stored-procedure statement types (C#, TypeScript, Rust)."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

ROW = [
    ("CreateCreateStoredProcedureStatement", False, "create_create"),
    ("CreateReadStoredProcedureStatement", True, "create_read"),
    ("CreateUpdateStoredProcedureStatement", True, "create_update"),
    ("CreateDeleteStoredProcedureStatement", True, "create_delete"),
]
BULK = [
    ("CreateBulkCreateStoredProcedureStatement", "create_bulk_create"),
    ("CreateBulkUpdateStoredProcedureStatement", "create_bulk_update"),
    ("CreateBulkDeleteStoredProcedureStatement", "create_bulk_delete"),
]


def rust_visit(snake: str) -> str:
    return f"visit_{snake}_stored_procedure"


def ts_visit(name: str) -> str:
    return f"visit{name}"


def cs_iface(name: str) -> str:
    return f"I{name}"


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def cs_iface_src(name: str) -> str:
    return f"""namespace Datasources;

public interface {cs_iface(name)} : IStatement;
"""


def cs_row(name: str, by_props: bool) -> str:
    iface = cs_iface(name)
    extra = "\n    IReadOnlyList<string> ByProperties," if by_props else ""
    return f"""namespace Datasources;

public sealed record {name}(
    string Name,
    string Table,
    IReadOnlyList<string> Columns,
    IReadOnlyList<RoutineParameter> Parameters,
    string Key,
    string KeyType,
    IReadOnlyList<string> Writable,{extra}
    bool GeneratedKey = false,
    string? AuditUpdated = null) : {iface}
{{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}}
"""


def cs_bulk(name: str) -> str:
    iface = cs_iface(name)
    return f"""namespace Datasources;

public sealed record {name}(
    string Name,
    string Table,
    IReadOnlyList<RoutineParameter> Columns,
    RoutineParameter Payload,
    string Key,
    string KeyType) : {iface}
{{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}}
"""


def cs_renderer(name: str) -> str:
    return f"""namespace Datasources;

public class {name}TextRenderer
{{
    public virtual string Render({name} statement) => "";
}}
"""


def ts_row(name: str, by_props: bool) -> tuple[str, str, str]:
    iface = cs_iface(name)
    extra_ctor = "\n    readonly byProperties: readonly string[] = []," if by_props else ""
    stmt = f"""import type {{ {iface} }} from "./{iface}.ts";
import type {{ IStatementVisitor }} from "./IStatementVisitor.ts";
import type {{ RoutineParameter }} from "./CreateRoutineStatement.ts";

export class {name} implements {iface} {{
  constructor(
    readonly name: string,
    readonly table: string,
    readonly columns: readonly string[],
    readonly parameters: readonly RoutineParameter[],
    readonly key: string,
    readonly keyType: string,
    readonly writable: readonly string[],{extra_ctor}
    readonly generatedKey = false,
    readonly auditUpdated?: string,
  ) {{}}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}}
"""
    iface_src = f"""import type {{ IStatement }} from "./IStatement.ts";

export interface {iface} extends IStatement {{}}
"""
    rend = f"""import type {{ {name} }} from "./{name}.ts";

export class {name}TextRenderer {{
  render(_statement: {name}): string {{
    return "";
  }}
}}
"""
    return stmt, iface_src, rend


def ts_bulk(name: str) -> tuple[str, str, str]:
    iface = cs_iface(name)
    stmt = f"""import type {{ {iface} }} from "./{iface}.ts";
import type {{ IStatementVisitor }} from "./IStatementVisitor.ts";
import type {{ RoutineParameter }} from "./CreateRoutineStatement.ts";

export class {name} implements {iface} {{
  constructor(
    readonly name: string,
    readonly table: string,
    readonly columns: readonly RoutineParameter[],
    readonly payload: RoutineParameter,
    readonly key: string,
    readonly keyType: string,
  ) {{}}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}}
"""
    iface_src = f"""import type {{ IStatement }} from "./IStatement.ts";

export interface {iface} extends IStatement {{}}
"""
    rend = f"""import type {{ {name} }} from "./{name}.ts";

export class {name}TextRenderer {{
  render(_statement: {name}): string {{
    return "";
  }}
}}
"""
    return stmt, iface_src, rend


def rust_row(name: str, snake: str, by_props: bool) -> tuple[str, str]:
    extra_field = "    by_properties: Vec<String>,\n" if by_props else ""
    extra_arg = "        by_properties: Vec<String>,\n" if by_props else ""
    extra_assign = "            by_properties,\n" if by_props else ""
    extra_get = (
        """
    pub fn by_properties(&self) -> &[String] {
        &self.by_properties
    }
"""
        if by_props
        else ""
    )
    visit = rust_visit(snake)
    stmt = f"""use super::create_routine_statement::RoutineParameter;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct {name} {{
    name: String,
    table: String,
    columns: Vec<String>,
    parameters: Vec<RoutineParameter>,
    key: String,
    key_type: String,
    writable: Vec<String>,
{extra_field}    generated_key: bool,
    audit_updated: Option<String>,
}}

impl {name} {{
    pub fn new(
        name: impl AsRef<str>,
        table: impl AsRef<str>,
        columns: Vec<String>,
        parameters: Vec<RoutineParameter>,
        key: impl AsRef<str>,
        key_type: impl AsRef<str>,
        writable: Vec<String>,
{extra_arg}        generated_key: bool,
        audit_updated: Option<String>,
    ) -> Self {{
        Self {{
            name: name.as_ref().to_string(),
            table: table.as_ref().to_string(),
            columns,
            parameters,
            key: key.as_ref().to_string(),
            key_type: key_type.as_ref().to_string(),
            writable,
{extra_assign}            generated_key,
            audit_updated,
        }}
    }}

    pub fn name(&self) -> &str {{
        &self.name
    }}

    pub fn table(&self) -> &str {{
        &self.table
    }}

    pub fn columns(&self) -> &[String] {{
        &self.columns
    }}

    pub fn parameters(&self) -> &[RoutineParameter] {{
        &self.parameters
    }}

    pub fn key(&self) -> &str {{
        &self.key
    }}

    pub fn key_type(&self) -> &str {{
        &self.key_type
    }}

    pub fn writable(&self) -> &[String] {{
        &self.writable
    }}
{extra_get}
    pub fn generated_key(&self) -> bool {{
        self.generated_key
    }}

    pub fn audit_updated(&self) -> Option<&str> {{
        self.audit_updated.as_deref()
    }}
}}

impl Statement for {name} {{
    fn accept(&self, text: &StatementTextRenderer) -> String {{
        text.statements().{visit}(self, text)
    }}

    fn as_any(&self) -> &dyn std::any::Any {{
        self
    }}
}}

impl crate::ReadInput for {name} {{
    fn as_statement(&self) -> Option<&dyn crate::Statement> {{
        Some(self)
    }}
}}
"""
    rend = f"""use super::{snake}_stored_procedure_statement::{name};

pub struct {name}TextRenderer;

impl {name}TextRenderer {{
    pub fn render(&self, _statement: &{name}) -> String {{
        String::new()
    }}
}}
"""
    return stmt, rend


def rust_bulk(name: str, snake: str) -> tuple[str, str]:
    visit = rust_visit(snake)
    stmt = f"""use super::create_routine_statement::RoutineParameter;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct {name} {{
    name: String,
    table: String,
    columns: Vec<RoutineParameter>,
    payload: RoutineParameter,
    key: String,
    key_type: String,
}}

impl {name} {{
    pub fn new(
        name: impl AsRef<str>,
        table: impl AsRef<str>,
        columns: Vec<RoutineParameter>,
        payload: RoutineParameter,
        key: impl AsRef<str>,
        key_type: impl AsRef<str>,
    ) -> Self {{
        Self {{
            name: name.as_ref().to_string(),
            table: table.as_ref().to_string(),
            columns,
            payload,
            key: key.as_ref().to_string(),
            key_type: key_type.as_ref().to_string(),
        }}
    }}

    pub fn name(&self) -> &str {{
        &self.name
    }}

    pub fn table(&self) -> &str {{
        &self.table
    }}

    pub fn columns(&self) -> &[RoutineParameter] {{
        &self.columns
    }}

    pub fn payload(&self) -> &RoutineParameter {{
        &self.payload
    }}

    pub fn key(&self) -> &str {{
        &self.key
    }}

    pub fn key_type(&self) -> &str {{
        &self.key_type
    }}
}}

impl Statement for {name} {{
    fn accept(&self, text: &StatementTextRenderer) -> String {{
        text.statements().{visit}(self, text)
    }}

    fn as_any(&self) -> &dyn std::any::Any {{
        self
    }}
}}

impl crate::ReadInput for {name} {{
    fn as_statement(&self) -> Option<&dyn crate::Statement> {{
        Some(self)
    }}
}}
"""
    rend = f"""use super::{snake}_stored_procedure_statement::{name};

pub struct {name}TextRenderer;

impl {name}TextRenderer {{
    pub fn render(&self, _statement: &{name}) -> String {{
        String::new()
    }}
}}
"""
    return stmt, rend


def main() -> None:
    cs_dir = ROOT / "csharp/Datasources/Statements"
    ts_dir = ROOT / "typescript/datasources/src/statements"
    rs_dir = ROOT / "rust/datasources/src/statements"

    rust_mods = []
    rust_uses = []
    ts_exports_type = []
    ts_exports_class = []
    cs_visits = []
    ts_visits = []
    ts_imports = []
    rust_visits = []
    rust_uses_renderer = []
    names = []

    for name, by_props, snake in ROW:
        names.append((name, snake, by_props, False))
        write(cs_dir / f"{name}.cs", cs_row(name, by_props))
        write(cs_dir / f"{cs_iface(name)}.cs", cs_iface_src(name))
        write(cs_dir / f"{name}TextRenderer.cs", cs_renderer(name))
        stmt, iface, rend = ts_row(name, by_props)
        write(ts_dir / f"{name}.ts", stmt)
        write(ts_dir / f"{cs_iface(name)}.ts", iface)
        write(ts_dir / f"{name}TextRenderer.ts", rend)
        s, r = rust_row(name, snake, by_props)
        write(rs_dir / f"{snake}_stored_procedure_statement.rs", s)
        write(rs_dir / f"{snake}_stored_procedure_statement_text_renderer.rs", r)
        rust_mods += [
            f"mod {snake}_stored_procedure_statement;",
            f"mod {snake}_stored_procedure_statement_text_renderer;",
        ]
        rust_uses += [
            f"pub use {snake}_stored_procedure_statement::{name};",
            f"pub use {snake}_stored_procedure_statement_text_renderer::{name}TextRenderer;",
        ]
        ts_exports_type.append(f'export type {{ {cs_iface(name)} }} from "./{cs_iface(name)}.ts";')
        ts_exports_class += [
            f'export {{ {name} }} from "./{name}.ts";',
            f'export {{ {name}TextRenderer }} from "./{name}TextRenderer.ts";',
        ]
        cs_visits.append(f"    string Visit({name} statement);")
        ts_imports.append(f'import type {{ {name} }} from "./{name}.ts";')
        ts_visits.append(f"  visit(statement: {name}): string;")
        rust_uses_renderer.append(
            f"use super::{snake}_stored_procedure_statement::{name};\nuse super::{snake}_stored_procedure_statement_text_renderer::{name}TextRenderer;"
        )
        rust_visits.append(
            f"""    fn {rust_visit(snake)}(
        &self,
        statement: &{name},
        _text: &StatementTextRenderer,
    ) -> String {{
        {name}TextRenderer.render(statement)
    }}"""
        )

    for name, snake in BULK:
        names.append((name, snake, False, True))
        write(cs_dir / f"{name}.cs", cs_bulk(name))
        write(cs_dir / f"{cs_iface(name)}.cs", cs_iface_src(name))
        write(cs_dir / f"{name}TextRenderer.cs", cs_renderer(name))
        stmt, iface, rend = ts_bulk(name)
        write(ts_dir / f"{name}.ts", stmt)
        write(ts_dir / f"{cs_iface(name)}.ts", iface)
        write(ts_dir / f"{name}TextRenderer.ts", rend)
        s, r = rust_bulk(name, snake)
        write(rs_dir / f"{snake}_stored_procedure_statement.rs", s)
        write(rs_dir / f"{snake}_stored_procedure_statement_text_renderer.rs", r)
        rust_mods += [
            f"mod {snake}_stored_procedure_statement;",
            f"mod {snake}_stored_procedure_statement_text_renderer;",
        ]
        rust_uses += [
            f"pub use {snake}_stored_procedure_statement::{name};",
            f"pub use {snake}_stored_procedure_statement_text_renderer::{name}TextRenderer;",
        ]
        ts_exports_type.append(f'export type {{ {cs_iface(name)} }} from "./{cs_iface(name)}.ts";')
        ts_exports_class += [
            f'export {{ {name} }} from "./{name}.ts";',
            f'export {{ {name}TextRenderer }} from "./{name}TextRenderer.ts";',
        ]
        cs_visits.append(f"    string Visit({name} statement);")
        ts_imports.append(f'import type {{ {name} }} from "./{name}.ts";')
        ts_visits.append(f"  visit(statement: {name}): string;")
        rust_uses_renderer.append(
            f"use super::{snake}_stored_procedure_statement::{name};\nuse super::{snake}_stored_procedure_statement_text_renderer::{name}TextRenderer;"
        )
        rust_visits.append(
            f"""    fn {rust_visit(snake)}(
        &self,
        statement: &{name},
        _text: &StatementTextRenderer,
    ) -> String {{
        {name}TextRenderer.render(statement)
    }}"""
        )

    # Patch C# visitor
    vis = (cs_dir / "IStatementVisitor.cs").read_text()
    vis = vis.replace(
        "    string Visit(DropRoutineStatement statement);\n}",
        "    string Visit(DropRoutineStatement statement);\n"
        + "\n".join(cs_visits)
        + "\n}",
    )
    write(cs_dir / "IStatementVisitor.cs", vis)

    rend = (cs_dir / "StatementRenderer.cs").read_text()
    extra = ""
    for name, _, _, _ in names:
        extra += f"""
    public virtual string Visit({name} statement) =>
        new {name}TextRenderer().Render(statement);
"""
    rend = rend.rstrip() + extra + "\n"
    write(cs_dir / "StatementRenderer.cs", rend)

    text = (cs_dir / "StatementTextRenderer.cs").read_text()
    arms = "".join(f"        {name} s => s.Accept(Statements),\n" for name, _, _, _ in names)
    text = text.replace(
        "        DropRoutineStatement s => s.Accept(Statements),\n",
        "        DropRoutineStatement s => s.Accept(Statements),\n" + arms,
    )
    write(cs_dir / "StatementTextRenderer.cs", text)

    # TS visitor
    vis = (ts_dir / "IStatementVisitor.ts").read_text()
    vis = vis.replace(
        'import type { DropRoutineStatement } from "./DropRoutineStatement.ts";\n',
        'import type { DropRoutineStatement } from "./DropRoutineStatement.ts";\n' + "\n".join(ts_imports) + "\n",
    )
    vis = vis.replace(
        "  visit(statement: DropRoutineStatement): string;\n}",
        "  visit(statement: DropRoutineStatement): string;\n" + "\n".join(ts_visits) + "\n}",
    )
    write(ts_dir / "IStatementVisitor.ts", vis)

    idx = (ts_dir / "index.ts").read_text()
    idx = idx.replace(
        'export type { IDropRoutineStatement } from "./IDropRoutineStatement.ts";\n',
        'export type { IDropRoutineStatement } from "./IDropRoutineStatement.ts";\n' + "\n".join(ts_exports_type) + "\n",
    )
    idx = idx.replace(
        'export { DropRoutineStatementTextRenderer } from "./DropRoutineStatementTextRenderer.ts";\n',
        'export { DropRoutineStatementTextRenderer } from "./DropRoutineStatementTextRenderer.ts";\n'
        + "\n".join(ts_exports_class)
        + "\n",
    )
    write(ts_dir / "index.ts", idx)

    # TS StatementRenderer
    sr = (ts_dir / "StatementRenderer.ts").read_text()
    imports = "".join(
        f'import {{ {name} }} from "./{name}.ts";\nimport {{ {name}TextRenderer }} from "./{name}TextRenderer.ts";\n'
        for name, _, _, _ in names
    )
    sr = sr.replace(
        'import { DropRoutineStatementTextRenderer } from "./DropRoutineStatementTextRenderer.ts";\n',
        'import { DropRoutineStatementTextRenderer } from "./DropRoutineStatementTextRenderer.ts";\n' + imports,
    )
    union = "  | DropRoutineStatement" + "".join(f"\n  | {name}" for name, _, _, _ in names)
    sr = sr.replace("  | DropRoutineStatement;", union + ";")
    overloads = "  visit(statement: DropRoutineStatement): string;\n" + "".join(
        f"  visit(statement: {name}): string;\n" for name, _, _, _ in names
    )
    sr = sr.replace("  visit(statement: DropRoutineStatement): string;\n", overloads)
    dispatch = "".join(
        f"      {name}: () => this.{ts_visit(name)}(statement as {name}),\n" for name, _, _, _ in names
    )
    sr = sr.replace(
        "      DropRoutineStatement: () => this.visitDropRoutine(statement as DropRoutineStatement),\n",
        "      DropRoutineStatement: () => this.visitDropRoutine(statement as DropRoutineStatement),\n" + dispatch,
    )
    methods = "".join(
        f"""
  protected {ts_visit(name)}(statement: {name}): string {{
    return new {name}TextRenderer().render(statement);
  }}
"""
        for name, _, _, _ in names
    )
    sr = sr.rstrip() + methods + "\n"
    write(ts_dir / "StatementRenderer.ts", sr)

    # rust mod
    mod = (rs_dir / "mod.rs").read_text()
    mod = mod.replace(
        "mod drop_routine_statement;\n",
        "mod drop_routine_statement;\n" + "\n".join(rust_mods) + "\n",
    )
    mod = mod.replace(
        "pub use drop_routine_statement_text_renderer::DropRoutineStatementTextRenderer;\n",
        "pub use drop_routine_statement_text_renderer::DropRoutineStatementTextRenderer;\n"
        + "\n".join(rust_uses)
        + "\n",
    )
    write(rs_dir / "mod.rs", mod)

    sren = (rs_dir / "statement_renderer.rs").read_text()
    sren = sren.replace(
        "use super::drop_routine_statement_text_renderer::DropRoutineStatementTextRenderer;\n",
        "use super::drop_routine_statement_text_renderer::DropRoutineStatementTextRenderer;\n"
        + "\n".join(rust_uses_renderer)
        + "\n",
    )
    sren = sren.replace(
        """    fn visit_drop_routine(
        &self,
        statement: &DropRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        DropRoutineStatementTextRenderer.render(statement)
    }
}""",
        """    fn visit_drop_routine(
        &self,
        statement: &DropRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        DropRoutineStatementTextRenderer.render(statement)
    }

"""
        + "\n".join(rust_visits)
        + "\n}",
    )
    write(rs_dir / "statement_renderer.rs", sren)

    lib = (ROOT / "rust/datasources/src/lib.rs").read_text()
    extra_lib = ", ".join(name for name, _, _, _ in names)
    extra_rend = ", ".join(f"{name}TextRenderer" for name, _, _, _ in names)
    lib = lib.replace(
        "CreateRoutineStatementTextRenderer,",
        f"CreateRoutineStatementTextRenderer, {extra_lib}, {extra_rend},",
    )
    write(ROOT / "rust/datasources/src/lib.rs", lib)

    ts_index = (ROOT / "typescript/datasources/src/index.ts").read_text()
    ifaces = ",\n  ".join(cs_iface(n) for n, _, _, _ in names)
    classes = ",\n  ".join(n for n, _, _, _ in names)
    rends = ",\n  ".join(f"{n}TextRenderer" for n, _, _, _ in names)
    ts_index = ts_index.replace(
        "  ICreateRoutineStatement,\n",
        f"  ICreateRoutineStatement,\n  {ifaces},\n",
    )
    ts_index = ts_index.replace(
        "  CreateRoutineStatement,\n  CreateRoutineStatementTextRenderer,\n",
        f"  CreateRoutineStatement,\n  CreateRoutineStatementTextRenderer,\n  {classes},\n  {rends},\n",
    )
    write(ROOT / "typescript/datasources/src/index.ts", ts_index)

    print("generated", len(names), "statement types")


if __name__ == "__main__":
    main()
