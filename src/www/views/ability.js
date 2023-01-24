import { abilities } from "../../assets/abilities.toml";
import "../styles/ability.css";

/**
 * Component for a single ability's icon, optionally with button binding.
 * @param {{ icon: string, binding?: string }} param0
 */
export function AbilityIcon({ id, binding }) {
    console.log(id);
    return (
        <div class="ability-icon">
            <img src={abilities[id].icon}/>
        </div>
    );
}