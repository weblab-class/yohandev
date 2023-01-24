import { useEffect, useRef, useMemo } from "preact/hooks";
import { game } from "../../platform/web";
import { AbilityIcon } from "./ability";

import "../styles/game.css";

export function Game() {
    const ref = useRef();
    const instance = useMemo(() => game(), []);

    // Attach the instance of the game to DOM:
    useEffect(() => {
        instance.then(({ hook }) => {
            hook(ref.current);
        });
    }, [ref]);

    return (
        <div ref={ref} class="w:100vw h:100vh">
            <div class="ability-bar row">
                <AbilityIcon id="push" binding="1"/>
                <AbilityIcon id="shotgun" binding="2"/>
                <AbilityIcon id="heal" binding="3"/>
                <AbilityIcon id="shield" binding="4"/>
            </div>
        </div>
    );
}