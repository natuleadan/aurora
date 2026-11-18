#!/usr/bin/env node
// Unified validation runner (Node, no shell scripts).
// Runs every check with a timeout, captures output, prints PASS/FAIL and
// exits non-zero if any check fails (G-27).
import { spawnSync } from "node:child_process";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";

const ROOT = process.cwd();
const TIMEOUT_MS = 300_000;
const MAX_LINES = 500;
const CHECKS = [];

function command(name, cmd, args) {
  CHECKS.push({ name, run: () => {
    const r = spawnSync(cmd, args, { cwd: ROOT, encoding: "utf8", timeout: TIMEOUT_MS });
    if (r.error) return { ok: false, output: String(r.error) };
    const output = ((r.stdout || "") + (r.stderr || "")).trim();
    return { ok: r.status === 0, output };
  } });
}

function walk(dir, out = []) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) {
      if (["target", "node_modules", ".git", ".nsh", "dist"].includes(name)) continue;
      walk(p, out);
    } else if (/\.(rs|md|sh)$/.test(name)) {
      out.push(p);
    }
  }
  return out;
}

function checkFileSize() {
  CHECKS.push({ name: "file size", run: () => {
    const bad = walk(ROOT)
      .map((f) => [f, readFileSync(f, "utf8").split("\n").length])
      .filter(([, n]) => n > MAX_LINES)
      .map(([f, n]) => `${relative(ROOT, f)} has ${n} lines`);
    return { ok: bad.length === 0, output: bad.join("\n") };
  } });
}

function checkGeneralization() {
  CHECKS.push({ name: "generalization", run: () => {
    const forbidden = [/\/Users\//, /\/home\//, /[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}/];
    const EXAMPLE = /@(example\.com|example\.org|domain\.com)$/;
    const bad = walk(ROOT)
      .filter((f) => !f.endsWith("ThirdPartyNotices.txt") && !f.includes("/tests/") && !f.includes("/test/"))
      .flatMap((f) => readFileSync(f, "utf8").split("\n").map((line, i) => [f, i + 1, line]))
      .filter(([, , line]) => forbidden.some((re) => re.test(line)) && !EXAMPLE.test(line))
      .map(([f, i, line]) => `${relative(ROOT, f)}:${i}: ${line.trim().slice(0, 80)}`);
    return { ok: bad.length === 0, output: bad.slice(0, 20).join("\n") };
  } });
}

// --- checks common to all repos ---
command("cargo fmt", "cargo", ["fmt", "--all", "--", "--check"]);
command("cargo clippy", "cargo", ["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"]);
command("cargo test", "cargo", ["test", "--workspace"]);
command("cargo check", "cargo", ["--config", 'resolver.min-publish-age="7 days"', "check", "--workspace"]);
command("notices", "node", ["helpers/tools/third-party-notices.mjs", "--check"]);
checkFileSize();
checkGeneralization();

// --- repo-specific checks are injected below ---

command("nsh check showcase", "nsh", ["check", "apps/showcase"]);
command("nsh test showcase", "nsh", ["test", "apps/showcase"]);
command("nsh check site", "nsh", ["check", "apps/site"]);
command("nsh test site", "nsh", ["test", "apps/site"]);
CHECKS.push({ name: "aurora-lint", run: () => {
  const dirs = ["apps/showcase/src/pages", "apps/site/src/pages", "crates/aurora-tokens/src",
    "crates/aurora-cn/src", "crates/aurora-primitives/src", "crates/aurora-registry/src",
    "crates/aurora-components/src", "crates/aurora-cli/src", "crates/aurora-test-support/src"];
  const files = ["apps/showcase/src/lib.rs", "apps/showcase/src/router.rs",
    "apps/showcase/src/shell.rs", "apps/showcase/src/ui.rs",
    "apps/site/src/lib.rs", "apps/site/src/router.rs", "apps/site/src/shell.rs"];
  for (const dir of dirs) {
    try { for (const f of readdirSync(join(ROOT, dir))) if (f.endsWith(".rs")) files.push(join(dir, f)); }
    catch { /* dir may not exist */ }
  }
  const r = spawnSync("cargo", ["run", "-q", "-p", "aurora-lint", "--", ...files],
    { cwd: ROOT, encoding: "utf8", timeout: TIMEOUT_MS });
  const output = ((r.stdout || "") + (r.stderr || "")).trim();
  return { ok: r.status === 0, output };
} });


let failures = 0;
for (const c of CHECKS) {
  const { ok, output } = c.run();
  if (ok) {
    console.log(`PASS ${c.name}`);
  } else {
    failures += 1;
    console.log(`FAIL ${c.name}`);
    if (output) console.log(output.split("\n").slice(-40).join("\n"));
  }
}
console.log(`TOTAL PASS=${CHECKS.length - failures} FAIL=${failures}`);
process.exit(failures === 0 ? 0 : 1);
