# Contacts CLI

The cwd is dest. Copy this sample onto `./`, then install Datasources, generate, build, and run.

`deterministic/settings.yaml` lists sqlite, mysql, and postgres, and generate writes SQL for each of those engines. The program prints `expected.txt`.

The tree in the repo today is `quick_starts/contacts_cli/csharp`. Rust and TypeScript tabs use the same command shape for those language folders.

## sqlite

::: dialects
```csharp
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
dotnet new console
cp -R quick_starts/contacts_cli/csharp/. ./
dotnet add package Datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run
```
```rust
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
cargo init
cp -R quick_starts/contacts_cli/rust/. ./
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run
```
```typescript
export PROVIDER=sqlite
export DATASOURCE_URL=sqlite:///./app.sqlite
npm init -y
cp -R quick_starts/contacts_cli/typescript/. ./
npm install @deterministic-code/datasources
npx @deterministic-code/datasources-cli setup --yes sqlite
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts
```
:::

## mysql, postgres

::: dialects
```csharp
export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
dotnet new console
cp -R quick_starts/contacts_cli/csharp/. ./
docker compose up -d --wait
dotnet add package Datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect csharp
dotnet build
dotnet run
```
```rust
export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
cargo init
cp -R quick_starts/contacts_cli/rust/. ./
docker compose up -d --wait
cargo add datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect rust
cargo run
```
```typescript
export PROVIDER=mysql
export DATASOURCE_URL=mysql://ds:ds@127.0.0.1:3306/datasources
npm init -y
cp -R quick_starts/contacts_cli/typescript/. ./
docker compose up -d --wait
npm install @deterministic-code/datasources
npx @deterministic-code/datasources-cli setup --yes mysql postgres
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts
export PROVIDER=postgres
export DATASOURCE_URL=postgres://ds:ds@127.0.0.1:5432/datasources
npx @deterministic-code/datasources-cli generate all --dialect typescript
npx tsx src/main.ts
```
:::
