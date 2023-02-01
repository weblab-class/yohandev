import { useEffect, useRef, useMemo, useState } from "preact/hooks";
import { game } from "../../platform/web";
import { AbilityIcon } from "./ability";
import { POST } from "../utils";

import { port } from "env";

import "../styles/game.css";

export function Game({ deck, playing }) {
    const ref = useRef();
    const instance = useMemo(() => game(port), []);
    const [cooldowns, setCooldowns] = useState([0, 0, 0, 0]);

    // Attach the instance of the game to DOM:
    useEffect(() => {
        instance.then(({ hook }) => {
            hook(ref.current, setCooldowns);
        });
    }, [ref]);
    // Send channel ID to server
    useEffect(() => {
        if (!playing) return;

        instance.then(({ uuid }) => {
            POST("/api/join-game", { uuid: uuid() });
        });
    }, [playing]);

    return (
        <div ref={ref} class="w:100vw h:100vh">
            <div class="ability-bar row">
                {deck.map((ability, i) => (
                    <AbilityIcon id={ability} binding={i + 1} key={i} cooldown={cooldowns[i]}/>    
                ))}
            </div>
        </div>
    );
}