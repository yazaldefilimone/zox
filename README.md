<samp>

- 04/2024:  Zox is 2x faster than [Bun](https://bun.sh/) in single-thread. I'm still working to make it multi-thread.

> At the moment, Zox depends on extremely large external libs and they are not the best in my view, but my current focus is to make Zox work and then I will optimize and do things my way, ex [swc](https://swc.rs/).

- [ ] Low-Level Bindings
- [ ] JavaScript minifier
  - [ ] Minify JavaScript identifier names
  - [ ] Compress JavaScript syntax (this is partially implemented)
  - [ ] Constant folding
  - [ ] Dead code elimination (this is partially implemented)
  - [ ] Read "sideEffects" from package.json
- [ ] Web APIs
- [ ] TypeScript
  - [ ] Transpiler
  - [ ] Type Checker
- [ ] Test Runner
- [ ] Module System
  - [ ] ES Modules
  - [ ] CommonJS
- [ ] File System
  - [ ] File Reader
    - [ ] To String
    - [ ] To JSON
    - [ ] To Buffer
  - [ ] File Writer
    - [ ] From String
    - [ ] From JSON
    - [ ] From Buffer
  - [ ] Shell Support (optional)
  - Blob Support
- [ ] Process
  - [ ] Arguments
  - [ ] Environment
  - [ ] Exit
  - [ ] PID
  - [ ] Platform
  - [ ] Stdin
  - [ ] Stdout
  - [ ] Stderr
- [ ] Network
  - [ ] HTTP Server
  - [ ] WebSocket Server
- [ ] Rust FFI
- [ ] Package Manager
- [ ] Web APIs

- [ ] Node.js Compatibility
  - [ ] Node-API support
  - [ ] child_process support
  - [ ] non-blocking "fs"
  - [ ] "net" module
  - [ ] "crypto" polyfill should use hardware-accelerated crypto for better performance
  - [ ] Finish "buffer" implementation
  - [ ] require implementation that natively supports ESM (rather than via transpiler). This would involve subclassing AbstractModuleRecord in  - [ ] JSC. This would better support lazy-loading CommonJS modules.






- The project isn't really a big deal, so I won't say much about it!





