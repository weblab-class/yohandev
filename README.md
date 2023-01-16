<div align="center">
    <h1><code>sentor</code></h1>
    A node editor for AI models.
</div>

## Captain's Logs
### Week 1
- Setup project template using `WebAssembly` + `Rust` + `Preact`.
    - Entry point is always `api/` which serves the frontend `www/`(and any other API routes) and (optionally) rebuilds it when modified.
    - Rust will be used for computationally expensive tasks, directly on the client.
- (WIP) Create a node editor taking API inspiration from `ReactFlow`.
    - Top-level `Graph` component with nodes/edges passed in as props.
    - Node moves, deletions, connections, etc. use the prop-callback pattern.
    - UI inspiration from Tailwind and GitHub's website.
    - Every node uses the `translate` CSS property for their global position.
    - All edges are part of one `svg` element, also with global positions.
    - Infinite workspace with scroll/zoom using `translate` + `scale` CSS properties.