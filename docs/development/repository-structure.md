ideal-atc/
├── apps/
├── configs/
├── docs/
├── packages/
├── proto/
├── scripts/
├── services/
├── simulation/
├── tests/
├── tools/
└── ...

Directory Responsibility
apps/ Operational user applications
services/ Backend/domain services
packages/ Reusable libraries/packages
proto/ Inter-service protocol contracts
simulation/ Synthetic traffic and scenarios
tests/ Cross-component verification
configs/ Environment/configuration profiles
scripts/ Developer automation
tools/ Development tooling
docs/ Engineering documentation

apps/
❌ Core surveillance algorithms
❌ ASTERIX decoding
❌ Database internals

services/
❌ UI components
❌ React-specific code

proto/
❌ Rust implementation
❌ Python implementation
❌ Business logic

packages/frontend/
❌ Backend state
❌ Database access
