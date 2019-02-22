---
name: New release
about: Choose this template when you want to publish a new version
title: Release [VERSION]
labels: ''
assignees: ''

---

# Prepare the release
- [ ] Bump the version numbers
- [ ] Update dependency versions for neighbouring crates
- [ ] Create a pull request

# Perform the release
- [ ] Merge the pull request
- [ ] Run `cargo publish`
- [ ] Tag the merge commit (e.g. '0.1.2')
