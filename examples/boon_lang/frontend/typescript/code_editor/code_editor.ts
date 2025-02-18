import { EditorState, Compartment } from '@codemirror/state'
import { EditorView, keymap } from '@codemirror/view'
import { basicSetup } from 'codemirror'
import { oneDark } from '@codemirror/theme-one-dark'
import { indentWithTab, defaultKeymap } from "@codemirror/commands"
import { indentUnit } from "@codemirror/language"

export class CodeEditorController {
    constructor() {}

    editor_view: EditorView | null = null
    on_change_handler = new Compartment

    init(parent_element: HTMLElement) {
        const min_height_editor = EditorView.theme({
            ".cm-content, .cm-gutter": { minHeight: "200px" },
            ".cm-content": { "font-family": "Fira Code" },
        })
        const state = EditorState.create({
            extensions: [
                basicSetup,
                oneDark,
                min_height_editor,
                keymap.of(defaultKeymap),
                keymap.of([indentWithTab]),
                indentUnit.of("    "),
                this.on_change_handler.of([])
            ],
        })
        this.editor_view = new EditorView({
            parent: parent_element,
            state,
        })
        this.editor_view.focus()
    }

    set_content(content: string) {
        if (this.editor_view!.state.doc.toString() !== content) {
            this.editor_view!.dispatch({
                changes: [
                    { from: 0, to: this.editor_view!.state.doc.length },
                    { from: 0, insert: content },
                ]
            })
        }
    }

    on_change(on_change: (content: string) => void) {
        const on_change_extension = EditorView.updateListener.of(view_update => {
            if (view_update.docChanged) {
                const document = view_update.state.doc.toString()
                on_change(document)
            }
        })
        this.editor_view!.dispatch({
            effects: this.on_change_handler.reconfigure(on_change_extension)
        })
    }
}
