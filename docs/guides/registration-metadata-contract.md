# Registration Metadata Contract

> **At a glance** — Functions, entry points, websocket services, Event Script plugins and
> graph fetch features all register **declaratively**: metadata attached at the definition
> site, discovered and resolved by the engine at boot. This page fixes the metadata model
> and its semantics **across languages**. How each language attaches the metadata —
> a Java annotation, a Rust attribute macro, a Python or TypeScript decorator — is an
> idiom; everything else on this page is the contract, and the
> [golden vectors](#conformance-the-golden-vectors) prove an engine honors it.

The Java engine is the reference implementation; its
[Annotations Reference](https://accenture.github.io/mercury-composable/guides/annotations-reference/)
documents the reference carrier surface in full. **This engine implements the contract
with the macro carrier** — the local [Macros Reference](macros-reference.md) is the
carrier twin of the annotations page. Python and Node.js ports implement it next.

## One model, many carriers

The same declaration in the four languages:

```rust
// Rust - link-time inventory (this engine)
#[preload(route = "hello.world", instances = 10, typed)]
struct HelloWorld;   // + impl TypedFunction<I, O>
```

```java
// Java - runtime classpath scan (the reference)
@PreLoad(route = "hello.world", instances = 10)
public class HelloWorld implements TypedLambdaFunction<Map<String, Object>, Object> { ... }
```

```python
# Python (future) - decorator + explicit package walk
@pre_load(route="hello.world", instances=10)
class HelloWorld(TypedLambdaFunction): ...
```

```typescript
// Node/TypeScript (future) - decorator + explicit module glob
@PreLoad({ route: "hello.world", instances: 10 })
class HelloWorld implements TypedLambdaFunction<EventEnvelope, unknown> { ... }
```

The discovery mechanics differ by necessity — only the JVM scans a classpath at runtime —
but the developer-visible style, the metadata model and the boot-time semantics must not.

## The canonical metadata model

Every registration resolves to one canonical record. Wire form for tooling and
conformance: JSON, camelCase keys, enums as strings (never ordinals).

```json
{
  "kind": "function | websocket | entrypoint-before | entrypoint-main | plugin | feature",
  "routes": ["array of route names - function kind; comma-separated aliases in the carrier"],
  "name": "string - websocket / plugin / feature kinds",
  "namespace": "string - websocket kind, default \"ws\"",
  "sequence": "integer - entrypoint kinds, default 10 (0 is framework-reserved)",
  "declaredInstances": "integer, default 1",
  "envInstances": "string configuration KEY, default \"\" - carried for audit",
  "resolvedInstances": "integer - the boot-resolved effective value, clamped 1..1000",
  "isPrivate": "boolean, default true",
  "zeroTracing": "boolean marker, default false",
  "eventInterceptor": "boolean marker, default false",
  "optionalService": "string | null - an OR-list of key, !key, key=value conditions"
}
```

### Capability fields (per-language applicability)

Some Java `@PreLoad` fields are capabilities of a runtime family, not universal contract:

| Field | Java | Rust | Python / Node guidance |
|---|---|---|---|
| `customSerializer` | runtime ObjectMapper swap per route | **N/A** — serde is the single compile-time serializer; per-type attributes cover the use cases | applicable — both have runtime serialization hooks; carry as a type-name string |
| `inputPojoClass` | defeats JVM type erasure; enables `List<PoJo>` | **N/A** — `TypedFunction<I, O>` is monomorphized; `body_as::<Vec<T>>()` carries the element type | Python: type hints suffice; Node: applicable as a class reference |
| `inputStrategy` / `outputStrategy` | SNAKE / CAMEL / DEFAULT, flippable at runtime (`snake.case.serialization`) | **N/A at runtime** — `#[serde(rename_all)]` fixes case at compile time (the one genuine capability difference) | applicable — runtime case mapping is natural in both |
| `executionHint` (reserved) | `@KernelThreadRunner` → kernel thread pool | reserved, unimplemented — every function is a tokio task; revisit if a field workload needs `spawn_blocking` | Python: relevant (GIL / thread pool); Node: worker_threads — design when porting |

!!! note "Rust port"
    The three N/A rows above ARE this port's dispositions, stated per the contract's own
    rule ("a port documents each N/A explicitly; silence is not a disposition"): serde
    replaces `customSerializer` and the runtime case strategies at compile time, and
    monomorphized `TypedFunction<I, O>` subsumes `inputPojoClass`. `executionHint` is
    reserved-only here, exactly as in Java.

## Fixed semantics — every port MUST

1. **Attach at definition, resolve at boot.** `env_instances` names a configuration KEY
   (which may itself hold `${ENV_VAR:default}`); the value is read at framework boot —
   never at decoration, macro-expansion or import time. A numeric value wins; anything
   else falls back to `declaredInstances`. Effective instances clamp to 1..1000.
2. **Optional-service grammar** is exactly the Java `Feature` evaluator: comma-separated
   conditions are OR-ed; `!` negates; `key=value` matches case-insensitively; a bare
   `key` means `key=true`; no marker means always required. Evaluated at boot against
   the application configuration; a false condition skips registration with a
   "Skip optional" log line.
3. **Marker stacking is order-free.** `#[zero_tracing]` / `#[event_interceptor]` /
   `#[optional_service]` combine with the primary attribute in any order, as Java
   annotations always have. Where a language's mechanism is order-sensitive by nature
   (Rust attribute macros expand outside-in; Python decorators apply bottom-up), the port
   owes the engineering to hide it — this engine's markers re-attach themselves so
   either order works.
4. **One conflict policy.** Explicit programmatic registration wins over declarative
   (it runs later and replaces). Within any registry, a duplicate name is a WARNING
   ("Reloading ... please check duplicated ... name") and the later registration wins —
   never a silent replace, never an error. Registration order among declaratively
   discovered items is not guaranteed (classpath scan order, link order, import order);
   do not rely on it.
5. **Extension-point naming.** A plugin or feature name is the positional string on the
   carrier — `#[simple_plugin("getFirst")]`, `#[fetch_feature("log-request-headers")]`,
   `@FetchFeature("log-request-headers")`. For plugins only, omitting the name derives
   it from the declaration: Java lowercases the first letter of the class simple name;
   Rust camelCases the snake_case fn name — **idiomatic declarations in every language
   yield the same registered name**. Plugins are Event Script capabilities (flow
   vocabulary): they take no optional-service gating and are never conditionally on/off.
   Features are runtime behaviors: they honor optional-service gating.
6. **Boot sequence**: discover → register → **override** (`yaml.preload.override`:
   configuration-driven route rename / fan-out / instance re-tuning, applied to the
   collected function set before registration —
   [ported here](configuration-reference.md#yamlpreloadoverride)) → resolve
   (`env_instances`) → validate (route uniqueness warnings, non-empty registries where
   built-ins are expected) → route table. Lifecycle anchors: plugins load before flow
   compilation; features load before graph execution; entry points run in ascending
   sequence with 0 reserved for the framework.
7. **Discovery is explicit and fails loudly.** Java: classpath scan over the base
   packages plus `web.component.scan` (free). Rust: link-time `inventory` collection —
   plus a startup assertion on expected built-in counts, because a crate that is never
   linked contributes nothing, silently. Python: a package walk (`pkgutil.walk_packages`)
   importing every module under the services packages **before** resolution — an
   unimported module's decorator never runs. Node: a directory glob + dynamic `import()`
   with the same failure mode. Both future ports must fail loudly on an empty registry
   that should not be empty.
8. **Misuse is a first-class contract.** Invalid declarations fail at the earliest
   possible stage with a helpful message — unknown parameters name the valid set, a
   marker without a primary points at the correct form. Where the failure is
   compile-time (Rust), compile-fail tests guard the messages (`trybuild`, the
   `tests/ui` suites); where it is import/boot-time (Java, Python, Node), unit tests do.

## Conformance — the golden vectors

The contract is proven the same way the
[event envelope wire format](https://accenture.github.io/mercury-composable/guides/event-envelope-wire-format/)
is: **golden vectors shared verbatim between repositories**. Three files, one per
metadata-rich kind:

| Vectors file | Kind | Java module | Rust crate |
|---|---|---|---|
| `registration-vectors/core.json` | function | platform-core | platform-core |
| `registration-vectors/plugin.json` | plugin | event-script-engine | event-script |
| `registration-vectors/feature.json` | feature | minigraph-playground-engine | knowledge-graph |

Each engine declares the same small fixture set through **its own carrier** (annotation /
macro / decorator), boots, and asserts that the resolved registrations match the golden
entries exactly — including boot-time `env_instances` resolution against the vectors'
`assumedConfig`, marker effects, name derivation, and the **absence** of every `gatedOut`
fixture. A new port passes the contract when all three vector suites pass against files
byte-identical to these. In this repository the suites are the `registration_vectors`
test binaries of the three crates (the vectors live under each crate's
`tests/resources/registration-vectors/`).

The websocket and entry-point kinds carry little metadata beyond a name/namespace or a
sequence; they are pinned by each engine's own registration tests rather than shared
vectors.

## Porting order (the playbook)

For a new language port, in order: (1) implement the carrier + local registry + boot
resolver for the function kind; (2) pass `core.json`; (3) add the extension points and
pass `plugin.json` / `feature.json`; (4) document every capability-field disposition and
intentionally unported carrier; (5) adopt the discovery-failure assertions. The
[future-ports playbook](../test-reports/event-over-http-interop.md) covers the runtime
side (wire format, telemetry parity) — this page covers the declaration side; together
they are the port-acceptance bar.

## See also

- [Macros Reference](macros-reference.md) — this engine's carrier surface
- [Annotations Reference](https://accenture.github.io/mercury-composable/guides/annotations-reference/) — the Java reference carrier
- ADR-0008 in `docs/arch-decisions/ADR.md` — this ledger's decision record (the Java
  ledger's ADR-0009 is its twin)

---

*Adapted from the mercury-composable guide `docs/guides/registration-metadata-contract.md`
(the reference copy of this contract); the contract content is identical — carrier
examples and links are localized to this repository.*
