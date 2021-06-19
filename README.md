# algorand-oas-generated-v2-clients

Rust clients for Algorand's `algod` v2 and `indexer` v1. Generated with the following command:

```bash
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli generate \
  -i /local/indexer.oas3.yml \
  -g rust \
  -o /local/indexer \
  --skip-validate-spec
```
