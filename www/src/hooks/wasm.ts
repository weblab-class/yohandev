import { useEffect, useState } from "preact/hooks";

type InstantiateFn = (_: WebAssembly.Imports) => Promise<WebAssembly.Exports>;

/**
 * Hook that accepts a WebAssembly instantiate function and returns a reactive
 * value that's `undefined` while loading and WebAssembly exports when loaded.
 */
export function useWasm(instantiate: InstantiateFn, imports: WebAssembly.Imports) {
    const [get, set] = useState<WebAssembly.Exports | undefined>();

    useEffect(() => {
        instantiate(imports)
            .then((exports) => set(exports));
    }, [instantiate]);

    return get;
}