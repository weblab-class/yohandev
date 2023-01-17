import { useWasm, useEffect, useRef, useMemo } from "../hooks";
import { SVG } from "@svgdotjs/svg.js";
import { geckos } from "@geckos.io/client";
import { port } from "argv";

import lib from "../lib.rs";

export function Game() {
    const root = useRef();
    const game = useWasm(lib, {
        /* Imports will go here. */
    });
    const draw = useMemo(() => SVG(), [root]);
    const server = useMemo(() => geckos({ port }), []);

    // Fullscreen canvas.
    useEffect(() => {
        draw.addTo(root.current)
            .size("100%", "100%");
    }, [root]);
    // Connect to server
    useEffect(() => {
        server.onConnect((e) => {
            // Error handling. 🤷🏽‍♂️
            if (e) throw e;

            console.log("Got connection!");
        });
    }, [server]);
    // Load game logic
    useEffect(() => {
        draw.text(`2+2=${game?.add(2, 2)}`)
            .x(Math.random() * 100)
            .y(Math.random() * 100)
            .stroke("red");
    }, [game]);

    // SVG.js will attach itself and handle lifecycle.
    return (<div ref={root} class="w:100vw h:100vh"/>);
}