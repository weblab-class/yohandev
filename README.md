<div align="center">
    <h1><code>sentor</code></h1>
    A node editor for AI models.
</div>

## Toolchain Dependencies
    - node
    - yarn(`brew install yarn`)
    - cmake(`brew install cmake`)
    - cargo([rustup.rs](https://rustup.rs))
        + `wasm32-unknown-unknown` target(`rustup target add wasm32-unknown-unknown`)
    - wabt(`brew install wabt`)
    - binaryen(`brew install binaryen`)

## Troubleshooting
    - darwin-arm64: Missing OpenSSL
        - You may need to force brew to link it. For me it was `brew link --force openssl@1.1`
        - Source: https://github.com/murat-dogan/node-datachannel/issues/63#issuecomment-1076034512

## Captain's Logs
### Week 1
- Setup project template using `WebAssembly` + `Rust` + `Preact`.
- Got sick
### Week 2
- As it turns out, daemons suck! I'd much rather have a shortcut in my IDE to build.

### Ideas
- Structuring networked ECS v0.2:
    - Define a list/enum of prefabs to spawn:
        ```
        pub struct Networked {
            id: Channel,    // Client that owns this entity
            prefab: enum {  // Prefab to spawn on other clients
                Player,
                Bullet,
                // etc...
            }
        }
        ```

- `Networked`() component

- Synchronization:
    - Each entity that needs to be replicated has a `Networked` component that specifies
    what data to send
    ```
    // Spawn a bullet:
    world.spawn((
        Sprite::Bullet,
        Transform::default(),
        Position::new(x, y),
        Velocity::new(dx, dy),
        Networked {
            replicate: Packet::SpawnBullet {
                pos: 
            },
            synchronize: // somehow send typeIDs of components to synchronize
                         // sender will send those component data as-is(via serialize trait)
                         // receiver will know how to receive via same method
        }
    ));
    ```

- Message system
    - ie. "spawn prefab X"
    - server sends it to itself for some system to respond, but another system intercepts it
    to network it
    ```
    pub fn spawn_players(world: &mut World, messages: Message<SpawnPlayer>) {
        // message either come locally or from the network
        //  -> decoupled netcode since server sends message to itself AND client
        //      -> client just receives the message over the network
        for message in messages {

        }
    }
    ```
