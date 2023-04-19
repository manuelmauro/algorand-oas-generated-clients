# Algorand OAS-Generated Clients

## Generate Clients

`algod` client generated with:

```bash
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli generate \
  -i /local/algod.oas3.yml \
  -g rust \
  -o /local/algod \
  --skip-validate-spec
```

`indexer` client generated with:

```bash
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli generate \
  -i /local/indexer.oas3.yml \
  -g rust \
  -o /local/indexer \
  --skip-validate-spec
```

## Fix Clients

VS Code regex

```regex
(format!.*)(a-id)(.*)(a-id)

$1a_id$3a_id
```

for `a` in `[application, asset, participation, account, ]`
