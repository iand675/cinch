// Runtime for AOT Cinch Script resources. All Godot values use the SDK ABI.
#include <cmath>
#include <godot_cpp/classes/engine.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/resource_format_loader.hpp>
#include <godot_cpp/classes/resource_format_saver.hpp>
#include <godot_cpp/classes/resource_loader.hpp>
#include <godot_cpp/classes/resource_saver.hpp>
#include <godot_cpp/classes/script_extension.hpp>
#include <godot_cpp/classes/script_language_extension.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/typed_array.hpp>
#include <memory>
#include <set>
#include <vector>
namespace cinch_scripts {
using namespace godot;
struct Field {
    StringName name;
    Variant::Type type;
    Variant initial;
    bool any = false;
};
struct Method {
    StringName name;
    std::vector<Variant::Type> args;
    Variant::Type result;
    int line = 1;
    bool returns_variant = false;
};
struct Signal {
    StringName name;
    std::vector<Variant::Type> args;
};
struct Template {
    String path, name, base, source;
    bool tool = false;
    std::vector<Field> fields;
    std::vector<Method> methods;
    std::vector<Signal> signals;
    void *(*create)(Object *);
    void (*destroy)(void *);
    bool (*get)(void *, const StringName &, Variant &);
    bool (*set)(void *, const StringName &, const Variant &);
    Variant (*call)(void *, const StringName &, const Variant **, int64_t, GDExtensionCallError &);
};
static std::vector<std::shared_ptr<Template>> templates;
static Dictionary property(const Field &f) {
    Dictionary p;
    p["name"] = f.name;
    p["type"] = f.type;
    p["hint"] = PROPERTY_HINT_NONE;
    p["hint_string"] = "";
    p["usage"] = PROPERTY_USAGE_DEFAULT | PROPERTY_USAGE_SCRIPT_VARIABLE |
                 (f.any ? PROPERTY_USAGE_NIL_IS_VARIANT : 0);
    return p;
}
static Dictionary method(const Method &m) {
    Dictionary d;
    d["name"] = m.name;
    d["flags"] = METHOD_FLAG_NORMAL;
    Array args;
    for (size_t i = 0; i < m.args.size(); ++i) {
        Field f{StringName(String("arg") + String::num_int64(i)), m.args[i], Variant(),
                m.args[i] == Variant::NIL};
        args.append(property(f));
    }
    d["args"] = args;
    d["default_args"] = Array();
    d["return"] = property(Field{StringName(), m.result, Variant(), m.returns_variant});
    return d;
}
static std::shared_ptr<Template> find_template(const String &path) {
    for (auto &t : templates)
        if (t->path == path)
            return t;
    return {};
}
static bool compatible(const Variant &v, Variant::Type expected) {
    return expected == Variant::NIL ||
           (expected == Variant::OBJECT && v.get_type() == Variant::NIL) ||
           Variant::can_convert_strict(v.get_type(), expected);
}
static void error(GDExtensionCallError &e, GDExtensionCallErrorType code, int arg = 0,
                  int expected = 0) {
    e.error = code;
    e.argument = arg;
    e.expected = expected;
}
class CinchLanguage;
static CinchLanguage *language = nullptr;
static void *language_owner();
class CinchScript : public ScriptExtension {
    GDCLASS(CinchScript, ScriptExtension);

  protected:
    static void _bind_methods() {
        ClassDB::bind_method(D_METHOD("new"), &CinchScript::new_instance);
    }

  public:
    Variant new_instance() {
        if (!_can_instantiate())
            return Variant();
        Variant value = ClassDB::instantiate(_get_instance_base_type());
        Object *owner = value;
        if (owner)
            owner->set_script(Ref<CinchScript>(this));
        return value;
    }
    std::shared_ptr<Template> compiled;
    String code;
    mutable std::set<uint64_t> owners;
    bool _editor_can_reload_from_file() override { return true; }
    bool _can_instantiate() const override {
        return _is_valid() && (compiled->tool || !Engine::get_singleton()->is_editor_hint());
    }
    Ref<Script> _get_base_script() const override { return {}; }
    StringName _get_global_name() const override {
        return compiled ? StringName(compiled->name) : StringName();
    }
    bool _inherits_script(const Ref<Script> &script) const override { return script.ptr() == this; }
    StringName _get_instance_base_type() const override {
        return compiled ? StringName(compiled->base) : StringName("Node");
    }
    void *_instance_create(Object *owner) const override;
    void *_placeholder_instance_create(Object *owner) const override;
    void _placeholder_erased(void *) override {}
    bool _instance_has(Object *o) const override { return o && owners.count(o->get_instance_id()); }
    bool _has_source_code() const override { return true; }
    String _get_source_code() const override { return code; }
    void _set_source_code(const String &s) override { code = s; }
    Error _reload(bool) override {
        if (!get_path().is_empty() && FileAccess::file_exists(get_path()))
            code = FileAccess::get_file_as_string(get_path());
        return _is_valid() ? OK : ERR_BUSY;
    }
    StringName _get_doc_class_name() const override { return _get_global_name(); }
    TypedArray<Dictionary> _get_documentation() const override { return {}; }
    String _get_class_icon_path() const override { return ""; }
    bool _has_method(const StringName &name) const override {
        if (compiled)
            for (auto &m : compiled->methods)
                if (m.name == name)
                    return true;
        return false;
    }
    bool _has_static_method(const StringName &) const override { return false; }
    Variant _get_script_method_argument_count(const StringName &name) const override {
        if (compiled)
            for (auto &m : compiled->methods)
                if (m.name == name)
                    return int64_t(m.args.size());
        return Variant();
    }
    Dictionary _get_method_info(const StringName &name) const override {
        if (compiled)
            for (auto &m : compiled->methods)
                if (m.name == name)
                    return method(m);
        return {};
    }
    bool _is_tool() const override { return compiled && compiled->tool; }
    bool _is_valid() const override { return compiled && code == compiled->source; }
    bool _is_abstract() const override { return false; }
    ScriptLanguage *_get_language() const override;
    bool _has_script_signal(const StringName &name) const override {
        if (compiled)
            for (auto &s : compiled->signals)
                if (s.name == name)
                    return true;
        return false;
    }
    TypedArray<Dictionary> _get_script_signal_list() const override {
        TypedArray<Dictionary> a;
        if (compiled)
            for (auto &s : compiled->signals)
                a.append(method(Method{s.name, s.args, Variant::NIL}));
        return a;
    }
    bool _has_property_default_value(const StringName &name) const override {
        if (compiled)
            for (auto &f : compiled->fields)
                if (f.name == name)
                    return true;
        return false;
    }
    Variant _get_property_default_value(const StringName &name) const override {
        if (compiled)
            for (auto &f : compiled->fields)
                if (f.name == name)
                    return f.initial;
        return Variant();
    }
    void _update_exports() override {}
    TypedArray<Dictionary> _get_script_method_list() const override {
        TypedArray<Dictionary> a;
        if (compiled)
            for (auto &m : compiled->methods)
                a.append(method(m));
        return a;
    }
    TypedArray<Dictionary> _get_script_property_list() const override {
        TypedArray<Dictionary> a;
        if (compiled)
            for (auto &f : compiled->fields)
                a.append(property(f));
        return a;
    }
    int32_t _get_member_line(const StringName &name) const override {
        if (compiled)
            for (auto &m : compiled->methods)
                if (m.name == name)
                    return m.line;
        return 1;
    }
    Dictionary _get_constants() const override { return {}; }
    TypedArray<StringName> _get_members() const override {
        TypedArray<StringName> a;
        if (compiled)
            for (auto &f : compiled->fields)
                a.append(f.name);
        return a;
    }
    bool _is_placeholder_fallback_enabled() const override { return false; }
    Variant _get_rpc_config() const override { return Dictionary(); }
};
struct Instance {
    Object *owner;
    uint64_t owner_id;
    Ref<CinchScript> script;
    void *state;
    String empty;
    StringName empty_name;
    std::vector<GDExtensionPropertyInfo> properties;
    std::vector<GDExtensionMethodInfo> methods;
    std::vector<std::vector<GDExtensionPropertyInfo>> arguments;
    std::vector<std::vector<StringName>> argument_names;
    Instance(Object *o, const Ref<CinchScript> &s)
        : owner(o), owner_id(o->get_instance_id()), script(s), state(s->compiled->create(o)) {
        auto &t = *script->compiled;
        script->owners.insert(o->get_instance_id());
        for (auto &f : t.fields)
            properties.push_back(info(f.name, f.type, f.any));
        arguments.resize(t.methods.size());
        argument_names.resize(t.methods.size());
        for (size_t i = 0; i < t.methods.size(); ++i) {
            auto &m = t.methods[i];
            auto &names = argument_names[i];
            names.reserve(m.args.size());
            for (size_t j = 0; j < m.args.size(); ++j)
                names.emplace_back(String("arg") + String::num_int64(j));
            for (size_t j = 0; j < m.args.size(); ++j)
                arguments[i].push_back(info(names[j], m.args[j], m.args[j] == Variant::NIL));
            GDExtensionMethodInfo mi{};
            mi.name = ptr(m.name);
            mi.return_value = info(empty_name, m.result, m.returns_variant);
            mi.flags = GDEXTENSION_METHOD_FLAG_NORMAL;
            mi.argument_count = m.args.size();
            mi.arguments = arguments[i].data();
            methods.push_back(mi);
        }
    }
    ~Instance() {
        script->owners.erase(owner_id);
        script->compiled->destroy(state);
    }
    static void *ptr(const StringName &name) {
        return const_cast<void *>(reinterpret_cast<const void *>(name._native_ptr()));
    }
    GDExtensionPropertyInfo info(const StringName &name, Variant::Type type, bool any) {
        GDExtensionPropertyInfo p{};
        p.type = GDExtensionVariantType(type);
        p.name = ptr(name);
        p.class_name = ptr(empty_name);
        p.hint = PROPERTY_HINT_NONE;
        p.hint_string = const_cast<void *>(reinterpret_cast<const void *>(empty._native_ptr()));
        p.usage = PROPERTY_USAGE_DEFAULT | PROPERTY_USAGE_SCRIPT_VARIABLE |
                  (any ? PROPERTY_USAGE_NIL_IS_VARIANT : 0);
        return p;
    }
};
static Instance &instance(void *p) { return *static_cast<Instance *>(p); }
static const StringName &name(GDExtensionConstStringNamePtr p) {
    return *reinterpret_cast<const StringName *>(p);
}
static void result(GDExtensionVariantPtr out, const Variant &value) {
    internal::gdextension_interface_variant_new_copy(out, value._native_ptr());
}
static const GDExtensionScriptInstanceInfo3 &instance_info() {
    static auto info = []() {
        GDExtensionScriptInstanceInfo3 i{};
        i.set_func = [](void *p, GDExtensionConstStringNamePtr n,
                        GDExtensionConstVariantPtr v) -> GDExtensionBool {
            auto &s = instance(p);
            return s.script->compiled->set(s.state, name(n), *reinterpret_cast<const Variant *>(v));
        };
        i.get_func = [](void *p, GDExtensionConstStringNamePtr n,
                        GDExtensionVariantPtr ret) -> GDExtensionBool {
            auto &s = instance(p);
            Variant v;
            if (!s.script->compiled->get(s.state, name(n), v))
                return false;
            result(ret, v);
            return true;
        };
        i.get_property_list_func = [](void *p, uint32_t *count) -> const GDExtensionPropertyInfo * {
            auto &s = instance(p);
            *count = s.properties.size();
            return s.properties.data();
        };
        i.free_property_list_func = [](void *, const GDExtensionPropertyInfo *, uint32_t) {};
        i.property_can_revert_func = [](void *p,
                                        GDExtensionConstStringNamePtr n) -> GDExtensionBool {
            return instance(p).script->_has_property_default_value(name(n));
        };
        i.property_get_revert_func = [](void *p, GDExtensionConstStringNamePtr n,
                                        GDExtensionVariantPtr ret) -> GDExtensionBool {
            auto &s = instance(p);
            if (!s.script->_has_property_default_value(name(n)))
                return false;
            result(ret, s.script->_get_property_default_value(name(n)));
            return true;
        };
        i.get_owner_func = [](void *p) -> GDExtensionObjectPtr {
            return instance(p).owner->_owner;
        };
        i.get_property_state_func = [](void *p, GDExtensionScriptInstancePropertyStateAdd add,
                                       void *userdata) {
            auto &s = instance(p);
            for (auto &f : s.script->compiled->fields) {
                Variant v;
                if (s.script->compiled->get(s.state, f.name, v))
                    add(f.name._native_ptr(), v._native_ptr(), userdata);
            }
        };
        i.get_method_list_func = [](void *p, uint32_t *count) -> const GDExtensionMethodInfo * {
            auto &s = instance(p);
            *count = s.methods.size();
            return s.methods.data();
        };
        i.free_method_list_func = [](void *, const GDExtensionMethodInfo *, uint32_t) {};
        i.get_property_type_func = [](void *p, GDExtensionConstStringNamePtr n,
                                      GDExtensionBool *valid) -> GDExtensionVariantType {
            auto &s = instance(p);
            for (auto &f : s.script->compiled->fields)
                if (f.name == name(n)) {
                    *valid = true;
                    return GDExtensionVariantType(f.type);
                }
            *valid = false;
            return GDEXTENSION_VARIANT_TYPE_NIL;
        };
        i.validate_property_func = [](void *, GDExtensionPropertyInfo *) -> GDExtensionBool {
            return false;
        };
        i.has_method_func = [](void *p, GDExtensionConstStringNamePtr n) -> GDExtensionBool {
            return instance(p).script->_has_method(name(n));
        };
        i.get_method_argument_count_func = [](void *p, GDExtensionConstStringNamePtr n,
                                              GDExtensionBool *valid) -> GDExtensionInt {
            auto &s = instance(p);
            for (auto &m : s.script->compiled->methods)
                if (m.name == name(n)) {
                    *valid = true;
                    return m.args.size();
                }
            *valid = false;
            return 0;
        };
        i.call_func = [](void *p, GDExtensionConstStringNamePtr n,
                         const GDExtensionConstVariantPtr *args, GDExtensionInt count,
                         GDExtensionVariantPtr ret, GDExtensionCallError *err) {
            auto &s = instance(p);
            error(*err, GDEXTENSION_CALL_OK);
            std::vector<const Variant *> native_args;
            for (int64_t j = 0; j < count; ++j)
                native_args.push_back(reinterpret_cast<const Variant *>(args[j]));
            Variant v = s.script->compiled->call(s.state, name(n), native_args.data(), count, *err);
            result(ret, v);
        };
        i.notification_func = [](void *p, int32_t what, GDExtensionBool) {
            auto &s = instance(p);
            if (s.script->_has_method("_notification")) {
                Variant value = what;
                const Variant *args[] = {&value};
                GDExtensionCallError e{};
                s.script->compiled->call(s.state, "_notification", args, 1, e);
            }
        };
        i.to_string_func = [](void *p, GDExtensionBool *valid, GDExtensionStringPtr out) {
            auto &s = instance(p);
            *valid = true;
            new (out) String(String("<Cinch ") + s.script->compiled->name + ">");
        };
        i.refcount_incremented_func = [](void *) {};
        i.refcount_decremented_func = [](void *) -> GDExtensionBool { return true; };
        i.get_script_func = [](void *p) -> GDExtensionObjectPtr {
            return instance(p).script->_owner;
        };
        i.is_placeholder_func = [](void *) -> GDExtensionBool { return false; };
        i.get_language_func = [](void *) -> GDExtensionScriptLanguagePtr {
            return language_owner();
        };
        i.free_func = [](void *p) { delete static_cast<Instance *>(p); };
        return i;
    }();
    return info;
}
void *CinchScript::_instance_create(Object *owner) const {
    if (!_can_instantiate() || !owner || !owner->is_class(compiled->base))
        return nullptr;
    auto *data = new Instance(owner, Ref<CinchScript>(const_cast<CinchScript *>(this)));
    return internal::gdextension_interface_script_instance_create3(&instance_info(), data);
}
void *CinchScript::_placeholder_instance_create(Object *owner) const {
    if (!compiled || !owner)
        return nullptr;
    auto *p = internal::gdextension_interface_placeholder_script_instance_create(
        language_owner(), _owner, owner->_owner);
    Array fields = _get_script_property_list();
    Dictionary defaults;
    for (auto &f : compiled->fields)
        defaults[f.name] = f.initial;
    internal::gdextension_interface_placeholder_script_instance_update(p, fields._native_ptr(),
                                                                       defaults._native_ptr());
    return p;
}
class CinchLanguage : public ScriptLanguageExtension {
    GDCLASS(CinchLanguage, ScriptLanguageExtension);

  protected:
    static void _bind_methods() {}

  public:
    String _get_name() const override { return "Cinch"; }
    void _init() override {}
    void _finish() override {}
    String _get_type() const override { return "CinchScript"; }
    String _get_extension() const override { return "cinch"; }
    PackedStringArray _get_reserved_words() const override {
        return {"module", "where", "import",    "export",   "foreign", "opaque", "type", "data",
                "record", "let",   "in",        "case",     "of",      "if",     "then", "else",
                "forall", "using", "signature", "instance", "True",    "False"};
    }
    bool _is_control_flow_keyword(const String &k) const override {
        return k == "if" || k == "then" || k == "else" || k == "case" || k == "of";
    }
    PackedStringArray _get_comment_delimiters() const override { return {"--", "{- -}"}; }
    PackedStringArray _get_doc_comment_delimiters() const override { return {"---"}; }
    PackedStringArray _get_string_delimiters() const override { return {"\" \"", "' '"}; }
    Ref<Script> _make_template(const String &, const String &klass, const String &) const override {
        Ref<CinchScript> s;
        s.instantiate();
        s->code = String("module ") + klass +
                  "Logic where\n  -- Add this file and its methods to bindings.json, then build.\n";
        return s;
    }
    TypedArray<Dictionary> _get_built_in_templates(const StringName &) const override { return {}; }
    bool _is_using_templates() override { return false; }
    Dictionary _validate(const String &code, const String &path, bool, bool, bool,
                         bool) const override {
        Dictionary d;
        auto t = find_template(path);
        d["valid"] = bool(t && code == t->source);
        Array errors;
        if (!t || code != t->source) {
            Dictionary e;
            e["line"] = 1;
            e["column"] = 1;
            e["message"] =
                "Build the Cinch project and restart Godot to load this source revision.";
            errors.append(e);
        }
        d["errors"] = errors;
        d["warnings"] = Array();
        PackedStringArray functions;
        if (t && code == t->source)
            for (auto &m : t->methods)
                functions.append(String(m.name) + String(":") + String::num_int64(m.line));
        d["functions"] = functions;
        d["safe_lines"] = PackedInt32Array();
        return d;
    }
    String _validate_path(const String &path) const override {
        return path.get_extension() == "cinch" ? String() : String("Cinch scripts use .cinch");
    }
    Object *_create_script() const override { return memnew(CinchScript); }
    bool _has_named_classes() const override { return true; }
    bool _supports_builtin_mode() const override { return false; }
    bool _supports_documentation() const override { return false; }
    bool _can_inherit_from_file() const override { return false; }
    int32_t _find_function(const String &fn, const String &code) const override {
        auto lines = code.split("\n");
        for (int i = 0; i < lines.size(); ++i)
            if (lines[i].strip_edges().begins_with(fn + String(" ::")))
                return i + 1;
        return -1;
    }
    String _make_function(const String &, const String &,
                          const PackedStringArray &) const override {
        return "";
    }
    bool _can_make_function() const override { return false; }
    Error _open_in_external_editor(const Ref<Script> &, int32_t, int32_t) override {
        return ERR_UNAVAILABLE;
    }
    bool _overrides_external_editor() override { return false; }
    Dictionary _complete_code(const String &, const String &, Object *) const override {
        Dictionary d;
        d["result"] = ERR_UNAVAILABLE;
        d["options"] = Array();
        d["force"] = false;
        d["call_hint"] = "";
        return d;
    }
    Dictionary _lookup_code(const String &, const String &, const String &,
                            Object *) const override {
        return {};
    }
    String _auto_indent_code(const String &code, int32_t, int32_t) const override { return code; }
    void _add_global_constant(const StringName &, const Variant &) override {}
    void _add_named_global_constant(const StringName &, const Variant &) override {}
    void _remove_named_global_constant(const StringName &) override {}
    void _thread_enter() override {}
    void _thread_exit() override {}
    String _debug_get_error() const override { return ""; }
    int32_t _debug_get_stack_level_count() const override { return 0; }
    int32_t _debug_get_stack_level_line(int32_t) const override { return -1; }
    String _debug_get_stack_level_function(int32_t) const override { return ""; }
    String _debug_get_stack_level_source(int32_t) const override { return ""; }
    Dictionary _debug_get_stack_level_locals(int32_t, int32_t, int32_t) override { return {}; }
    Dictionary _debug_get_stack_level_members(int32_t, int32_t, int32_t) override { return {}; }
    void *_debug_get_stack_level_instance(int32_t) override { return nullptr; }
    Dictionary _debug_get_globals(int32_t, int32_t) override { return {}; }
    String _debug_parse_stack_level_expression(int32_t, const String &, int32_t, int32_t) override {
        return "";
    }
    TypedArray<Dictionary> _debug_get_current_stack_info() override { return {}; }

    void _reload_all_scripts() override {}
    void _reload_scripts(const Array &, bool) override {}
    void _reload_tool_script(const Ref<Script> &script, bool keep) override {
        if (script.is_valid())
            script->reload(keep);
    }
    PackedStringArray _get_recognized_extensions() const override { return {"cinch"}; }
    TypedArray<Dictionary> _get_public_functions() const override { return {}; }
    Dictionary _get_public_constants() const override { return {}; }
    TypedArray<Dictionary> _get_public_annotations() const override { return {}; }
    void _profiling_start() override {}
    void _profiling_stop() override {}
    void _profiling_set_save_native_calls(bool) override {}
    void _frame() override {}
    bool _handles_global_class_type(const String &type) const override {
        return type == "CinchScript" || type == "Cinch";
    }
    Dictionary _get_global_class_name(const String &path) const override {
        Dictionary d;
        if (auto t = find_template(path)) {
            d["name"] = t->name;
            d["base_type"] = t->base;
            d["icon_path"] = "";
        }
        return d;
    }
};
static void *language_owner() { return language ? language->_owner : nullptr; }
ScriptLanguage *CinchScript::_get_language() const { return language; }
class CinchLoader : public ResourceFormatLoader {
    GDCLASS(CinchLoader, ResourceFormatLoader);

  protected:
    static void _bind_methods() {}

  public:
    PackedStringArray _get_recognized_extensions() const override { return {"cinch"}; }
    bool _recognize_path(const String &path, const StringName &) const override {
        return path.get_extension() == "cinch";
    }
    bool _handles_type(const StringName &type) const override {
        return type == StringName("CinchScript") || type == StringName("Script");
    }
    String _get_resource_type(const String &path) const override {
        return path.get_extension() == "cinch" ? "CinchScript" : "";
    }
    String _get_resource_script_class(const String &path) const override {
        auto t = find_template(path);
        return t ? t->name : String();
    }
    int64_t _get_resource_uid(const String &) const override { return -1; }
    bool _exists(const String &path) const override { return FileAccess::file_exists(path); }
    Variant _load(const String &path, const String &, bool, int32_t) const override {
        if (!FileAccess::file_exists(path))
            return ERR_FILE_NOT_FOUND;
        Ref<CinchScript> s;
        s.instantiate();
        s->compiled = find_template(path);
        s->code = FileAccess::get_file_as_string(path);
        s->set_path(path);
        return s;
    }
};
class CinchSaver : public ResourceFormatSaver {
    GDCLASS(CinchSaver, ResourceFormatSaver);

  protected:
    static void _bind_methods() {}

  public:
    bool _recognize(const Ref<Resource> &r) const override {
        return Object::cast_to<CinchScript>(r.ptr()) != nullptr;
    }
    bool _recognize_path(const Ref<Resource> &r, const String &path) const override {
        return _recognize(r) && path.get_extension() == "cinch";
    }
    PackedStringArray _get_recognized_extensions(const Ref<Resource> &r) const override {
        return _recognize(r) ? PackedStringArray{"cinch"} : PackedStringArray();
    }
    Error _save(const Ref<Resource> &r, const String &path, uint32_t) override {
        auto *script = Object::cast_to<CinchScript>(r.ptr());
        if (!script)
            return ERR_INVALID_PARAMETER;
        Ref<FileAccess> f = FileAccess::open(path, FileAccess::WRITE);
        if (f.is_null())
            return ERR_CANT_OPEN;
        f->store_string(script->code);
        return OK;
    }
};
static Ref<CinchLoader> loader;
static Ref<CinchSaver> saver;
// CINCH_SCRIPT_TABLES
static void initialize() {
    ClassDB::register_class<CinchScript>();
    ClassDB::register_class<CinchLanguage>();
    ClassDB::register_class<CinchLoader>();
    ClassDB::register_class<CinchSaver>();
    make_templates();
    language = memnew(CinchLanguage);
    if (Engine::get_singleton()->register_script_language(language) != OK)
        cinch_gd_bridge::fault();
    loader.instantiate();
    saver.instantiate();
    ResourceLoader::get_singleton()->add_resource_format_loader(loader, true);
    ResourceSaver::get_singleton()->add_resource_format_saver(saver, true);
}
static void terminate() {
    if (loader.is_valid())
        ResourceLoader::get_singleton()->remove_resource_format_loader(loader);
    if (saver.is_valid())
        ResourceSaver::get_singleton()->remove_resource_format_saver(saver);
    loader.unref();
    saver.unref();
    if (language) {
        Engine::get_singleton()->unregister_script_language(language);
        memdelete(language);
        language = nullptr;
    }
    templates.clear();
}
} // namespace cinch_scripts
