---
name: New release
about: Choose this template when you want to publish a new version
title: Release [VERSION]
labels: ''
assignees: ''

---

# Prepare a release PR
- [ ] Bump the version numbers
- [ ] Update dependency versions for neighbouring crates

# Perform the release
- [ ] Merge the PR
- [ ] Run `cargo publish`
- [ ] Tag the merge commit
