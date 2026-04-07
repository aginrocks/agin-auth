## Self-Review Checklist:

- [ ] I've split up large pieces of code into smaller, reusable pieces
- [ ] No logic is duplicated that could be abstracted into a function, method, hook or middleware
- [ ] I've commented my code where needed and avoided comments that provide no value

### Frontend checklist

- [ ] I've checked my code for duplicate components or UI elements that already exist elsewhere in the codebase
- [ ] Data fetching and mutations use React Query
- [ ] Forms use `useForm`
- [ ] I've formatted my code using Prettier

### Backend checklist

- [ ] Logic is not reimplemented manually when a suitable crate already exists (e.g. `openidconnect`, `axum-client-ip`, `serde_urlencoded`)
- [ ] I've used the `validator` crate for API input validation
- [ ] I've formatted my code using rustfmt
- [ ] My code produces no new warnings with Clippy enabled
