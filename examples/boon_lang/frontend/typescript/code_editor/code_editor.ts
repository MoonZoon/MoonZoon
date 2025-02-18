import { EditorState, Compartment } from '@codemirror/state'
import { EditorView, keymap } from '@codemirror/view'
import { basicSetup } from 'codemirror'
import { oneDark } from '@codemirror/theme-one-dark'
import { indentWithTab, defaultKeymap } from "@codemirror/commands"
import { indentUnit } from "@codemirror/language"

export class CodeEditorController {
    constructor() {}

    editor_view: EditorView | null = null
    file_content = new Compartment()

    set_content(content: string) {
        this.editor_view!.dispatch({
            changes: [
                { from: 0, to: this.editor_view!.state.doc.length },
                { from: 0, insert: content },
            ]
        })
    }

    async init(parent_element: HTMLElement) {
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
            ],
            doc: `------------ Hello world example ------------

-- Display text on HTML document
document: Document/new(root: 'Hello world!')`
        })

        this.editor_view = new EditorView({
            parent: parent_element,
            state,
        });
        EditorView.theme
    }
}
