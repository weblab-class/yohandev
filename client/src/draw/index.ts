import { Shape, Svg } from "@svgdotjs/svg.js";
import { Exports, f32, u32 } from "../../../shared/src/types";

/** An ECS entity. */
type Entity = u32;

export function imports(draw: Svg) {
    // Construct a new `Entity` -> `Shape` map.
    function cache<T extends Shape>(ctor: () => T) {
        const map: { [_: Entity]: T } = {};
        // Get or instantiate fn:
        return function(e: Entity) {
            if (!map.hasOwnProperty(e)) {
                map[e] = ctor();
            }
            return map[e];
        }
    }
    // Cache of entities and their respective SVG.
    const caches = {
        "rect": cache(() => draw.rect()),
        "circle": cache(() => draw.circle()),
    };
    return {
        iter_sprites_rect(e: Entity, x: f32, y: f32, w: f32, h: f32) {
            caches["rect"](e)
                .x(x)
                .y(y)
                .width(w)
                .height(h);
        },
        iter_sprites_circle(e: Entity, x: f32, y: f32, r: f32) {
            caches["circle"](e)
                .x(x)
                .y(y)
                .radius(r);
        },
    }
}

export function hook(exports: Exports & {
    /** No exports */
}) {

}