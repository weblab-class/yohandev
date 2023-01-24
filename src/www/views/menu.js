import { AbilityIcon } from "./ability";

import "../styles/menu.css";

/**
 * Main menu component.
 */
export function Menu({ ...props }) {
    return (
        <div {...props}>
            {/* Black overlay */}
            <div
                class="w:100vw h:100vh"
                style="background: rgb(0, 0, 0, 0.2)"
            />
            {/* Floating window */}
            <div class="menu centered column">
                <div class="row">
                    <span class="logo">bbox</span>
                    <span class="sign-in">Sign-in</span>
                </div>
                <div class="row">
                    <div class="ability-deck row">
                        <AbilityIcon id="push" size={72}/>
                        <AbilityIcon id="lightning" size={72}/>
                        <AbilityIcon id="grappling" size={72}/>
                        <AbilityIcon id="dual-gun" size={72}/>
                    </div>
                    <div class="player-preview">

                    </div>
                </div>
                <div class="row">
                    <div class="ability-collection">
                        {/* TODO: fetch users' abilities from db */}
                        {Array(20).fill().map((_, i) => (
                            <AbilityIcon id="shotgun" key={i} size={72}/>
                        ))}
                    </div>
                    <div class="column">
                        <div class="player-stats">

                        </div>
                        <button class="play">Play</button>
                    </div>
                </div>
            </div>
        </div>
    );
}