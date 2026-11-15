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
    check_values()
    await check_scripts()
    print("CINCH_GODOT_OK")
    quit(0)

func check_values():
    var values = ClassDB.instantiate("CinchValues")
    var other = ClassDB.instantiate("CinchValues")
    assert(values.label == "Cinch λ 🌍")
    assert(values.enabled and values.offset == Vector2(1, 2))
    assert(values.items == [1, "two", {"nested": true}])
    assert(values.table == {"hello": "world"})
    assert(values.payload == {"key": [1, 2]})
    values.items.append(99)
    assert(other.items.size() == 3, "collection defaults must be independent")
    var text = "λ😀 Unicode\n\"quote\""
    assert(values.echoString(text) == text)
    assert(values.echoStringName(&"Playerλ") == &"Playerλ")
    assert(values.echoBool(true) and not values.echoBool(false))
    assert(values.echoVector2(Vector2(-2.5, 4)) == Vector2(-2.5, 4))
    assert(values.echoVector3(Vector3(1, 2, 3)) == Vector3(1, 2, 3))
    assert(values.echoVector4(Vector4(1, 2, 3, 4)) == Vector4(1, 2, 3, 4))
    assert(values.echoVector2i(Vector2i(-2, 4)) == Vector2i(-2, 4))
    assert(values.echoVector3i(Vector3i(1, 2, 3)) == Vector3i(1, 2, 3))
    assert(values.echoVector4i(Vector4i(1, 2, 3, 4)) == Vector4i(1, 2, 3, 4))
    assert(values.echoColor(Color(0.25, 0.5, 0.75, 1)) == Color(0.25, 0.5, 0.75, 1))
    assert(values.exerciseCollections() == 42)
    var original = {"nested": [1, 2]}
    var duplicate = values.duplicateDictionary(original)
    duplicate.nested.append(3)
    assert(original.nested == [1, 2] and duplicate.nested == [1, 2, 3])
    var resource = Resource.new()
    var callable = Callable(self, "receive_changed")
    var nested = [null, false, -42, 1.25, text, Vector3(1, 2, 3), resource, callable, PackedByteArray([0, 255]), {"key": [1, 2]}]
    values.emitVariant(nested) # emitting before any connection is valid
    var messages: Array = []
    values.connect("delivered", func(payload): messages.append(payload))
    values.emitVariant(nested)
    assert(messages == [nested])
    var delivered = values.get_signal_list().filter(func(info): return info.name == "delivered")[0]
    assert(delivered.args[0].usage & PROPERTY_USAGE_NIL_IS_VARIANT)
    assert(values.echoVariant(nested) == nested)
    assert(values.echoVariant(null) == null)
    assert(values.echoArray(nested) == nested)
    assert(values.echoDictionary({"nested": nested}) == {"nested": nested})
    assert(values.echoObject(resource) == resource and values.echoObject(null) == null)
    var array = [1]
    assert(values.arrayAppend(array, "two") == [1, "two"])
    assert(array == [1, "two"], "arrays preserve Godot reference semantics")
    var node = Node2D.new()
    values.setObject(node, "position", Vector2(5, 6))
    assert(values.getObject(node, "position") == Vector2(5, 6))
    assert(values.callObject(node, "get_position", []) == Vector2(5, 6))
    values.callObject(node, "translate", [Vector2(1, 2)])
    assert(node.position == Vector2(6, 8))
    node.free()
    # Stress rich callback ownership, including temporary Cinch strings and records.
    for i in 1000:
        assert(values.echoString("λ😀" + str(i)) == "λ😀" + str(i))
        assert(values.echoVariant(nested) == nested)

var received: Array = []
func receive_changed(count, label):
    received.append([count, label])

func check_scripts():
    var probe = ClassDB.instantiate("CinchSpinner")
    var before = probe.destructions()
    var script = load("res://Mover.cinch")
    assert(script is Script and script.can_instantiate())
    assert(script.get_instance_base_type() == &"Node2D")
    assert(script.get_global_name() == &"CinchMover")
    assert(script.has_script_signal("changed"))
    var first = Node2D.new()
    var second = Node2D.new()
    first.set_script(script)
    second.set_script(script)
    assert(script.instance_has(first))
    assert(first.count == 1 and second.count == 1)
    first.label = "Unicode λ🌍"
    assert(first.advance(2) == 3 and second.count == 1)
    assert(first.connect_changed(self, "receive_changed", 0) == OK)
    first.advance(4)
    assert(received == [[7, "Unicode λ🌍"]])
    assert(first.disconnect_changed(self, "receive_changed") == OK)
    first.advance(1)
    assert(received.size() == 1)
    assert(first.connect_changed(self, "receive_changed", CONNECT_DEFERRED) == OK)
    first.advance(1)
    assert(received.size() == 1)
    await process_frame
    assert(received.back() == [9, "Unicode λ🌍"])
    first.disconnect_changed(self, "receive_changed")
    var watcher = Node.new()
    watcher.set_script(load("res://signal_receiver.gd"))
    assert(first.connect_changed(watcher, "receive_changed", CONNECT_DEFERRED) == OK)
    first.advance(1)
    watcher.free() # queued calls and connections must not retain a freed receiver
    await process_frame
    root.add_child(first)
    await process_frame
    await process_frame
    assert(first.count > 10 and first.position.x > 0, "script frame callback/inherited calls")
    first.free()
    second.free()
    var scene = load("res://mover.tscn").instantiate()
    assert(scene.get_script() == script and scene.count == 12 and scene.label == "Saved in scene")
    root.add_child(scene)
    await process_frame
    assert(scene.count >= 12)
    scene.free()
    assert(probe.destructions() == before + 3, "one destruction per script instance")
    var resource_script = load("res://ResourceLogic.cinch")
    var resource = resource_script.new()
    assert(resource is Resource and resource.label == "Resource λ")
    assert(resource.echo("hello λ😀") == "hello λ😀")
    resource = null
    assert(probe.destructions() == before + 4, "reference counting must destroy script state")
    var constructed = CinchMover.new()
    assert(constructed is Node2D and constructed.advance(1) == 2)
    constructed.free()
    assert(probe.destructions() == before + 5)
    probe.free()
    var tool = Node2D.new()
    tool.set_script(load("res://Tool.cinch"))
    assert(tool.count == 42)
    root.add_child(tool)
    assert(tool.count == 99)
    tool.free()
    # Saving source is supported; stale revisions must never execute the old native code.
    var source = script.source_code
    script.source_code = source + "\n-- edited in Godot\n"
    assert(not script.can_instantiate())
    assert(ResourceSaver.save(script, "user://cinch_saved.cinch") == OK)
    assert(FileAccess.get_file_as_string("user://cinch_saved.cinch") == script.source_code)
    script.source_code = source
    assert(script.can_instantiate())
