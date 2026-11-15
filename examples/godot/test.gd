extends SceneTree

func _initialize():
    call_deferred("run_test")

func run_test():
    assert(ClassDB.class_exists("CinchSpinner"), "Cinch class was not registered")
    var first = ClassDB.instantiate("CinchSpinner")
    var second = ClassDB.instantiate("CinchSpinner")
    assert(first.advance(2) == 2)
    assert(second.count == 0, "instances must have independent fields")
    first.count = 10
    assert(first.advance(1) == 11, "inspector property writes must reach Cinch")
    root.add_child(first)
    root.add_child(second)
    await process_frame
    await process_frame
    assert(first.count > 11, "Godot did not call Cinch _process")
    assert(first.rotation > 0, "Cinch did not call the inherited Godot rotate method")
    first.free()
    assert(second.destructions() == 1, "Cinch destructor callback did not run")
    second.free()
    print("CINCH_GODOT_OK")
    quit(0)
