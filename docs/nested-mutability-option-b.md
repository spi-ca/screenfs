# Nested mutability option B proposal

이 문서는 현재 canonical contract를 즉시 바꾸는 구현 완료 선언이 아니라, future mutability 확장안 중 **option B: 제한적 nested override**를 정리한 제안서다.

현재 구현/문서의 canonical contract는 다음을 유지한다.

- mount당 mutability family는 하나만 선택한다.
- `selective-readonly`와 `readonly-root-allowwrite` 두 family로 닫는다.
- `--readonly-rule`와 `--allow-write`는 현재 함께 사용할 수 없다.
- hidden path는 어떤 mutability policy보다 우선해 `ENOENT`다.

이 문서는 위 current contract를 대체하지 않는다. 다만 향후 유스케이스가 충분히 강할 때, **family 자체는 유지하면서 family 내부에서 nested exception을 허용하는 방향**을 spec 초안 수준으로 정리한다.

## 1. 왜 필요한가

현재 surface는 아래 두 요구를 직접 표현하지 못한다.

1. `selective-readonly`에서 readonly subtree 아래 일부만 다시 writable
2. `readonly-root-allowwrite`에서 writable carve-out 아래 일부만 다시 readonly

예시:

- `/workspace`는 readonly지만 `/workspace/tmp`만 writable
- 전체 mount는 readonly지만 `/workspace`는 writable, 단 `/workspace/vendor`는 다시 readonly

현행 모델에서는 이런 “예외의 예외”를 mount 하나로 표현할 수 없다.

## 2. 제안 범위

option B는 **general ordered policy language**로 재설계하지 않는다. 다음 제약을 유지한다.

- mutability family는 계속 `selective-readonly`와 `readonly-root-allowwrite` 두 개로 닫는다.
- 한 mount는 계속 정확히 하나의 family만 선택한다.
- hide semantics와 hidden `ENOENT` precedence는 변경하지 않는다.
- rule input normalization contract는 hide/current rule surfaces와 계속 공유한다.
- 기존 `--readonly` legacy surface를 복원하지 않는다.

즉, option B는 “family 제거”가 아니라 **family 내부에서 opposite-polarity exception rule을 허용**하는 제한적 확장이다.

## 3. 사용자-facing 목표

### 3.1 `selective-readonly`

기본 정책은 그대로 유지한다.

- visible path는 기본 writable
- readonly rule에 매치된 path는 `EROFS`

확장 규칙:

- readonly subtree 아래에 더 구체적인 `allow-write` rule을 둘 수 있다.
- 더 구체적인 carve-out은 해당 하위 subtree에서만 `EROFS`를 해제한다.

예시:

```text
family = selective-readonly
readonly:    /workspace
allow-write: /workspace/tmp
```

기대 결과:

- `/workspace/docs/a.txt` write -> `EROFS`
- `/workspace/tmp/out.txt` write -> 허용
- `/workspace` 밖 visible path -> 기본 writable

### 3.2 `readonly-root-allowwrite`

기본 정책도 그대로 유지한다.

- visible path는 기본 readonly
- allow-write rule에 매치된 path만 writable 후보

확장 규칙:

- allow-write subtree 아래에 더 구체적인 `readonly` rule을 둘 수 있다.
- 더 구체적인 readonly 재차단은 해당 하위 subtree에서 다시 `EROFS`를 적용한다.

예시:

```text
family = readonly-root-allowwrite
allow-write: /workspace
readonly:    /workspace/vendor
```

기대 결과:

- `/workspace/out.txt` write -> 허용
- `/workspace/vendor/lock.json` write -> `EROFS`
- `/free/out.txt` write -> `EROFS`

## 4. 핵심 의미론

### 4.1 hidden precedence

가장 먼저 유지할 규칙:

- hidden path 또는 hidden symlink target이 하나라도 관여하면 결과는 mutability rule과 무관하게 `ENOENT`
- hidden `ENOENT`는 nested override보다 항상 우선한다.

### 4.2 rule polarity

각 family는 primary polarity와 secondary polarity를 가진다.

- `selective-readonly`
  - primary: `readonly`
  - secondary: `allow-write`
- `readonly-root-allowwrite`
  - primary: `allow-write`
  - secondary: `readonly`

secondary polarity는 **primary rule의 하위 carve-out/re-block** 용도로만 해석한다.

### 4.3 precedence rule

option B의 추천 precedence는 다음과 같다.

1. hidden `ENOENT` 우선
2. mutability rule끼리는 **most-specific-match wins**
3. specificity가 같고 polarity가 충돌하면 fail-fast
4. 어떤 rule도 매치하지 않으면 family default를 사용

specificity 정의 초안:

- exact path가 prefixed glob보다 더 구체적이다.
- 더 긴 normalized path prefix가 더 구체적이다.
- 동일한 normalized target set을 가리키는 중복/충돌 rule은 fail-fast 후보다.

문서와 구현은 “rule 선언 순서”보다 “normalized specificity”를 우선 source of truth로 삼는 편이 안전하다. 그래야 CLI 반복 옵션과 config list 모두에서 일관된 판정이 가능하다.

### 4.4 operation-aware evaluation

현재 프로젝트의 affected-coordinate model은 유지한다.

- write-intent `open`
- `write`
- `create`, `mkdir`, `mknod`
- `unlink`, `rmdir`
- `rename`, `link`, `symlink`
- mutation `setattr`
- `fallocate`
- `copy_file_range`

허용 조건:

- mutation에 필요한 **모든 write-requiring coordinate**가 최종 판정상 writable이어야 한다.
- 하나라도 readonly면 전체 operation은 `EROFS`다.
- 하나라도 hidden이면 전체 operation은 `ENOENT`다.

`copy_file_range`는 현재 contract와 같은 분리를 유지한다.

- source: hidden/read visibility 대상으로 평가
- destination path + destination parent: writability 대상으로 평가

## 5. family별 세부 규칙

### 5.1 `selective-readonly`

default:

- 매치 rule이 없으면 writable

final writability 판정:

- 가장 구체적인 매치가 `readonly`면 `EROFS`
- 가장 구체적인 매치가 `allow-write`면 writable
- 단, `allow-write`는 hidden을 뒤집지 못한다.

권장 제약:

- secondary `allow-write` rule은 primary `readonly` rule과 disjoint하게 두는 것보다, primary subtree 내부 carve-out으로 쓸 때만 의미가 있다고 문서화한다.
- primary ancestor가 없는 secondary rule 자체를 금지할지, 허용하되 no-op처럼 볼지는 명시해야 한다.

추천안:

- `selective-readonly`에서 primary ancestor 없이 standalone `allow-write` rule이 들어오면 **fail-fast**
- 이유: 기본값이 이미 writable이므로 standalone `allow-write`는 의미가 없고 오해를 만든다.

### 5.2 `readonly-root-allowwrite`

default:

- 매치 rule이 없으면 readonly

final writability 판정:

- 가장 구체적인 매치가 `allow-write`면 writable
- 가장 구체적인 매치가 `readonly`면 `EROFS`
- 단, `readonly`도 hidden을 뒤집지 못한다.

권장 제약:

- secondary `readonly` rule은 primary `allow-write` subtree 내부 re-block 용도로만 허용한다.

추천안:

- `readonly-root-allowwrite`에서 primary ancestor 없이 standalone `readonly` rule이 들어오면 **fail-fast**
- 이유: 기본값이 이미 readonly이므로 standalone `readonly`는 의미가 없고 오해를 만든다.

## 6. CLI surface 초안

현재 CLI는 `--readonly-rule`와 `--allow-write` 동시 사용을 금지한다. option B에서는 이 금지를 완화하되, family-aware validation을 더 강하게 해야 한다.

기본 원칙:

- `--policy-family`는 계속 명시 가능
- family를 생략하면 현재처럼 추론할 수 있지만, option B에서는 추론 ambiguity가 커지므로 **둘 다 쓰는 경우 family 명시를 요구**하는 편이 안전하다.

추천 CLI contract:

### 6.1 `selective-readonly`

허용:

```bash
screenfs /src /mnt \
  --policy-family selective-readonly \
  --readonly-rule /workspace \
  --allow-write /workspace/tmp
```

검증:

- `--allow-write`는 최소 하나의 `--readonly-rule` 하위에 있어야 한다.
- 같은 specificity의 readonly/allow-write 충돌 rule은 fail-fast
- primary ancestor 없는 `--allow-write`는 fail-fast

### 6.2 `readonly-root-allowwrite`

허용:

```bash
screenfs /src /mnt \
  --policy-family readonly-root-allowwrite \
  --allow-write /workspace \
  --readonly-rule /workspace/vendor
```

검증:

- `--readonly-rule`는 최소 하나의 `--allow-write` 하위에 있어야 한다.
- 같은 specificity의 allow-write/readonly 충돌 rule은 fail-fast
- primary ancestor 없는 `--readonly-rule`는 fail-fast

### 6.3 family 추론

추천안:

- `--readonly-rule`만 있으면 `selective-readonly`
- `--allow-write`만 있으면 `readonly-root-allowwrite`
- 둘 다 있으면 `--policy-family` 필수

이렇게 해야 “둘 다 있는 입력”을 polarity만 보고 자동 해석하다가 잘못 추론하는 문제를 줄일 수 있다.

## 7. Config surface 초안

현재 config contract는 다음 block을 사용한다.

```yaml
mutability:
  family: selective-readonly | readonly-root-allowwrite
  readonly_rules:
    - ...
  allow_write:
    - ...
```

option B에서도 이 block shape는 유지할 수 있다. 대신 validation을 강화한다.

예시 1: selective-readonly + carve-out

```yaml
mutability:
  family: selective-readonly
  readonly_rules:
    - /workspace
  allow_write:
    - /workspace/tmp
```

예시 2: readonly-root-allowwrite + re-block

```yaml
mutability:
  family: readonly-root-allowwrite
  allow_write:
    - /workspace
  readonly_rules:
    - /workspace/vendor
```

config validation rules:

- family마다 primary/secondary polarity 제약을 동일하게 적용한다.
- secondary rule은 최소 하나의 primary ancestor를 가져야 한다.
- 같은 specificity 충돌은 fail-fast다.
- CLI mutability option이 하나라도 있으면 현재처럼 config `mutability` block 전체를 대체한다.

## 8. normalization contract

option B에서도 다음 계약은 유지해야 한다.

- hide / readonly / allow-write는 같은 normalization contract를 공유한다.
- exact path, supported prefixed glob, unsupported wildcard, `HOME`/`source_root`/`~user` fail-fast semantics를 일관되게 유지한다.
- precedence는 **정규화 이후의 virtual path / compiled glob prefix 기준**으로 계산한다.

특히 중요한 점:

- raw input 문자열 순서나 표기 차이(`./x`, `/src/../src/x`, `~/proj/x`)가 아니라
- 정규화된 virtual absolute path/prefix를 기준으로 specificity를 계산해야 한다.

## 9. 구현 관점 영향

현재 `RuntimeConfig::is_readonly()`는 family switch 하나로 단순하다.

```text
SelectiveReadonly => matches_readonly_rule(path)
ReadonlyRootAllowwrite => !matches_allow_write_rule(path)
```

option B로 가면 이 판정은 다음 정보가 필요해진다.

- primary rule 매치 결과
- secondary rule 매치 결과
- 각 매치의 specificity
- conflict/fail-fast 여부
- operation별 affected coordinate 전체 평가

즉, 단순 bool matcher 2개만으로는 부족할 가능성이 높고, 최소한 다음 수준의 evaluator가 필요하다.

- `classify_mutability(path) -> hidden | writable | readonly | invalid-conflict`
- 또는 `best_match(path)`가 polarity + specificity를 함께 반환

하지만 이것이 ordered general policy engine까지 의미하지는 않는다. option B 범위에서는 family와 polarity 제약을 유지한 evaluator면 충분하다.

## 10. QA 및 테스트 요구

option B를 도입하면 최소 다음 검증이 필요하다.

### 10.1 parser / config validation

- `selective-readonly` + primary readonly + nested allow-write 허용
- `readonly-root-allowwrite` + primary allow-write + nested readonly 허용
- secondary rule without primary ancestor fail-fast
- equal-specificity conflict fail-fast
- both rule types present without `--policy-family` fail-fast

### 10.2 normalization

- absolute exact
- relative exact
- `~/...` exact
- supported prefixed glob
- relative prefixed glob
- `~/...` prefixed glob
- unsupported wildcard fail-fast

### 10.3 mutability semantics

- hidden beats any nested override
- most-specific-match wins
- non-matching path falls back to family default
- secondary carve-out/re-block only in descendant scope

### 10.4 operation coverage

- write-intent `open`
- `create`, `mkdir`, `unlink`, `rename`, `link`, `symlink`
- mutation `setattr`, `fallocate`, xattr
- `copy_file_range` source/destination split

### 10.5 live smoke

최소 두 mount smoke가 필요하다.

1. `selective-readonly` + nested `allow-write`
2. `readonly-root-allowwrite` + nested `readonly`

각 smoke는 다음을 포함해야 한다.

- allowed mutation success
- re-blocked mutation `EROFS`
- hidden path `ENOENT`
- non-match family default 확인
- conflict/fail-fast stderr artifact

## 11. compatibility 및 문서화 주의점

option B는 현재 계약과 호환되지 않는 부분이 있다.

- 현재는 `--readonly-rule` + `--allow-write` 동시 사용이 무조건 에러다.
- option B를 도입하면 이 금지는 “항상 금지”에서 “family-aware, ancestor-aware, conflict-aware validation”으로 바뀐다.

따라서 문서에는 반드시 아래를 분리해서 적어야 한다.

- **현재 canonical contract**
- **future option B proposal**
- pre-removal historical evidence는 archival-only라는 점

현재 구현 완료처럼 쓰면 안 된다.

## 12. 추천 결론

이 저장소의 현재 spec 방향과 위험도를 감안하면 다음 순서를 추천한다.

1. 현재 canonical contract는 그대로 유지한다.
2. nested override 유스케이스가 실제 consumer에서 반복 확인되면 option B를 별도 버전 spec으로 승격한다.
3. 그때도 ordered general policy language(option C)로 바로 가지 말고, 먼저 family-preserving nested override만 도입한다.

즉, recommendation은 다음과 같다.

- **지금 당장 구현 계약 변경 없음**
- **future 확장안으로 option B 문서화 가능**
- **precedence는 hidden-first + most-specific-match-wins + equal-specificity-conflict-fail-fast 권장**

## 13. Candidate canonical spec wording

아래 문구는 **현재 canonical contract를 즉시 교체하는 선언이 아니라**, option B를 채택하기로 결정했을 때 `README.md`, `docs/requirements.md`, `docs/design.md`에 옮길 수 있는 후보 문안이다.

### 13.1 Requirements-style wording candidate

> `ScreenFS`는 mutability policy family를 `selective-readonly`와 `readonly-root-allowwrite` 두 개로 유지한다. 한 mount는 정확히 하나의 family만 선택한다. hidden path와 hidden target은 어떤 mutability rule보다 우선해 `ENOENT`다. option B를 채택한 family-aware nested override contract에서는 family 내부 opposite-polarity rule을 descendant carve-out 또는 re-block 용도로만 허용한다. `selective-readonly` family에서는 readonly rule 하위의 더 구체적인 allow-write rule이 해당 descendant subtree에서만 `EROFS`를 해제할 수 있고, `readonly-root-allowwrite` family에서는 allow-write rule 하위의 더 구체적인 readonly rule이 해당 descendant subtree를 다시 `EROFS`로 되돌릴 수 있다. mutability precedence는 hidden-first 다음 most-specific-match-wins를 따르며, same-specificity polarity conflict는 fail-fast다. secondary rule은 최소 하나의 primary ancestor가 있어야 하며, 그렇지 않으면 invalid configuration이다. write-intent open, path-only mutation, multi-path mutation, destination mutation이 있는 `copy_file_range`를 포함한 모든 mutation은 operation별 모든 write-requiring coordinate가 최종 판정상 writable일 때만 허용된다.

### 13.2 CLI/config wording candidate

> Canonical mutability CLI/config surface는 explicit family/rule contract를 유지한다. option B adopted contract에서는 `--readonly-rule`와 `--allow-write`를 같은 mount에서 함께 사용할 수 있지만, 이는 explicit `--policy-family`가 있는 경우에만 허용한다. `selective-readonly` family에서는 `--allow-write`가 최소 하나의 ancestor `--readonly-rule` 아래에 있어야 하고, `readonly-root-allowwrite` family에서는 `--readonly-rule`가 최소 하나의 ancestor `--allow-write` 아래에 있어야 한다. 동일 specificity 충돌 rule, primary ancestor 없는 secondary rule, normalization 후 동일 target-set polarity conflict는 모두 fail-fast다. config `mutability` block은 같은 family-aware validation을 따르며, CLI mutability option이 하나라도 있으면 config `mutability` block 전체를 대체한다.

### 13.3 Short README-style wording candidate

> Future option B proposal: keep one-family-per-mount semantics, but allow nested opposite-polarity exceptions within the selected family. Hidden `ENOENT` remains highest priority, mutability uses most-specific-match-wins, same-specificity conflicts fail fast, and secondary rules are only valid under a primary ancestor of the selected family.

## 14. 짧은 문서용 요약

한 줄 요약:

> option B는 mutability family를 유지한 채, family 내부에서 더 구체적인 반대 polarity rule을 nested exception으로 허용하는 제한적 확장안이다.

핵심 bullet:

- one-family-per-mount 유지
- hidden `ENOENT` 최우선 유지
- normalization contract 공유 유지
- `selective-readonly`: readonly 아래 allow-write carve-out 허용
- `readonly-root-allowwrite`: allow-write 아래 readonly re-block 허용
- precedence: most-specific-match wins
- same-specificity conflict: fail-fast
- secondary rule without primary ancestor: fail-fast
