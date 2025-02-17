import { EditorState, Compartment, Extension } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { basicSetup } from 'codemirror'
import { oneDark } from '@codemirror/theme-one-dark'

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
        const minHeightEditor = EditorView.theme({
            ".cm-content, .cm-gutter": {minHeight: "200px"}
        })
        const state = EditorState.create({
            extensions: [
                basicSetup,
                oneDark,
                minHeightEditor
            ],
            doc: `document: 
    Duration[seconds: 1]
    |> Timer/interval() 
    |> THEN { 1 } 
    |> Math/sum()
    |> Document/new()`
        })

        this.editor_view = new EditorView({
            parent: parent_element,
            state,
        });
        EditorView.theme
    }
}
