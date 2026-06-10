# Mermaid diagram artifacts

`docs/diagrams`는 Mermaid 다이어그램의 저장소 기준 source of truth다.

- 원본: `*.mmd`
- 공용 Mermaid 설정: `mermaid-config.json`
- 공용 Puppeteer 설정: `puppeteer-config.json`
- 산출물: 같은 basename의 `*.svg`, `*.png`

## Config 역할

- `mermaid-config.json`: theme, font, color 같은 Mermaid 렌더링 스타일을 정의한다.
- `puppeteer-config.json`: Mermaid CLI가 Chromium/Puppeteer를 띄울 때 사용할 launch args를 정의한다.
- 주의: 이 두 JSON만으로는 PNG 2x scale이 자동 보장되지 않는다. PNG 해상도 규칙은 렌더 명령에서 Mermaid CLI `--scale 2`로 강제한다.

## 저장소 기준 렌더링 규칙

- SVG는 위 두 config를 함께 사용해 렌더링한다.
- PNG는 같은 config를 사용하되 반드시 `--scale 2`를 추가한다.
- 새 다이어그램을 추가하거나 `*.mmd`, `mermaid-config.json`, `puppeteer-config.json`을 바꾸면 대응 `*.svg`, `*.png`를 같이 재생성한다.

## 예시 명령

단일 SVG:

```bash
npx -y @mermaid-js/mermaid-cli \
  -i docs/diagrams/system-context.mmd \
  -o docs/diagrams/system-context.svg \
  -c docs/diagrams/mermaid-config.json \
  -p docs/diagrams/puppeteer-config.json
```

단일 PNG(2x 필수):

```bash
npx -y @mermaid-js/mermaid-cli \
  -i docs/diagrams/system-context.mmd \
  -o docs/diagrams/system-context.png \
  -c docs/diagrams/mermaid-config.json \
  -p docs/diagrams/puppeteer-config.json \
  --scale 2
```

전체 다이어그램 재생성:

```bash
for input in docs/diagrams/*.mmd; do
  base="${input%.mmd}"

  npx -y @mermaid-js/mermaid-cli \
    -i "$input" \
    -o "${base}.svg" \
    -c docs/diagrams/mermaid-config.json \
    -p docs/diagrams/puppeteer-config.json

  npx -y @mermaid-js/mermaid-cli \
    -i "$input" \
    -o "${base}.png" \
    -c docs/diagrams/mermaid-config.json \
    -p docs/diagrams/puppeteer-config.json \
    --scale 2

done
```

## 확인

재생성 후에는 적어도 다음을 확인한다.

```bash
file docs/diagrams/*.png
```

그리고 문서 diff에서 `docs/architecture.md`, `docs/design.md`, `README.md`가 같은 규칙을 가리키는지 검토한다.
