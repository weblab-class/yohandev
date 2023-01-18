import { useEffect, useMemo, useRef } from "preact/hooks";
import { game } from "../lib";
import { port } from "argv";


export function Game() {
    const root = useRef();
    const instance = useMemo(() => game(port), []);

    useEffect(() => {
        // Add the game directly to the DOM to permit external
        // mutations outside the reactive lifecycle.
        instance.then((self) => {
            self.hook(root.current);
        });
    }, []);

    return (<div ref={root} class="w:100vw h:100vh"/>);
}