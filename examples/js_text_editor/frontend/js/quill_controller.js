export class QuillController {
    constructor(element) {
        this.quill = new Quill(element, {
            placeholder: 'Compose an epic...',
            theme: 'snow'
        });
    }

    on_change(on_change) {
        this.quill.on('text-change', () => {
            let delta = this.quill.getContents();
            on_change(JSON.stringify(delta));
        });
    }
}
