
    export function set_property(obj, name, value) { obj[name] = value; }

    export function add_event(elem, name, capture, passive, f) {
        elem.addEventListener(name, f, {
            capture,
            passive,
            once: false,
        });
    }

    export function add_event_once(elem, name, f) {
        elem.addEventListener(name, f, {
            capture: true,
            passive: true,
            once: true,
        });
    }

    export function remove_event(elem, name, capture, f) {
        elem.removeEventListener(name, f, capture);
    }
