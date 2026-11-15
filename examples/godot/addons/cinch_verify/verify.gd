@tool
extends EditorPlugin

func _enter_tree():
    call_deferred("verify")

func verify():
    assert(Engine.is_editor_hint())
    var helper = load("res://GameLogic.cinch")
    assert(helper is Script and helper.reload() == OK, "compiled helper modules must validate in the editor")
    assert(not helper.can_instantiate(), "a helper module is not an attached script class")
    var mover = load("res://Mover.cinch")
    var tool = load("res://Tool.cinch")
    assert(mover is Script and not mover.can_instantiate())
    assert(tool.can_instantiate())
    var placeholder = Node2D.new()
    placeholder.set_script(mover)
    assert(placeholder.count == 1)
    placeholder.count = 17
    placeholder.label = "Inspector λ"
    assert(placeholder.count == 17 and placeholder.label == "Inspector λ")
    placeholder.name = "SavedPlaceholder"
    var packed = PackedScene.new()
    assert(packed.pack(placeholder) == OK)
    assert(ResourceSaver.save(packed, "user://cinch_editor_scene.tscn") == OK)
    var restored = load("user://cinch_editor_scene.tscn").instantiate()
    assert(restored.count == 17 and restored.label == "Inspector λ")
    restored.free()
    placeholder.free()
    var node = Node2D.new()
    node.set_script(tool)
    assert(node.count == 42, "tool constructor must execute in editor")
    get_tree().root.add_child(node)
    assert(node.count == 99 and node.advance(3) == 102)
    node.free()
    EditorInterface.edit_script(mover)
    EditorInterface.set_main_screen_editor("Script")
    for i in 10:
        await get_tree().process_frame
    var editor = EditorInterface.get_script_editor().get_current_editor().get_base_editor()
    assert(editor is CodeEdit)
    assert(editor.is_indent_using_spaces() and editor.get_indent_size() == 2, "Cinch indentation must stay spaces on save")
    assert(editor.get_syntax_highlighter().is_class("CinchHighlighter"), "registered Cinch highlighting must be selected automatically")
    var edit = CodeEdit.new()
    get_tree().root.add_child(edit)
    var highlighter = ClassDB.instantiate("CinchHighlighter")
    edit.syntax_highlighter = highlighter
    edit.text = "module Game where\n  text = \"-- λ😀\" -- comment\n  {- outer\n    {- nested -}\n  end -} value :: I64\n  n = 42\n  char = '\\''\n"
    var first = highlighter.get_line_syntax_highlighting(0)
    var keyword_color = first[0].color
    assert(first.has(6) and first.has(7) and first[7].color != keyword_color)
    var quoted = highlighter.get_line_syntax_highlighting(1)
    assert(quoted.has(9) and quoted.has(16) and quoted.has(17))
    assert(quoted[9].color != quoted[17].color, "comment marker inside string must stay a string")
    var comment = highlighter.get_line_syntax_highlighting(2)[2].color
    assert(highlighter.get_line_syntax_highlighting(3)[0].color == comment)
    assert(highlighter.get_line_syntax_highlighting(4)[0].color == comment)
    assert(highlighter.get_line_syntax_highlighting(4).has(8))
    assert(highlighter.get_line_syntax_highlighting(5).has(6))
    edit.text = "module Replaced where\n  x = 1"
    assert(highlighter.get_line_syntax_highlighting(1).has(6), "edits must invalidate multiline state")
    edit.text = "  sym = 'hello\n  ty = ''Thing\n  char = 'x'\n  x--tail\n"
    assert(highlighter.get_line_syntax_highlighting(0).has(8))
    assert(highlighter.get_line_syntax_highlighting(1).has(7))
    assert(highlighter.get_line_syntax_highlighting(2).has(9))
    assert(highlighter.get_line_syntax_highlighting(3)[3].color == comment)
    edit.free()
    print("CINCH_GODOT_EDITOR_OK")
    get_tree().quit(0)
