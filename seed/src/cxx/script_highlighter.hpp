// Editor integration is initialized separately; exported games never access editor singletons.
#include <godot_cpp/classes/code_edit.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/editor_plugin.hpp>
#include <godot_cpp/classes/editor_plugin_registration.hpp>
#include <godot_cpp/classes/editor_syntax_highlighter.hpp>
#include <godot_cpp/classes/script_editor.hpp>
#include <godot_cpp/classes/text_edit.hpp>
#include <map>
#include <string>
#include <unordered_set>
namespace cinch_scripts {
class CinchHighlighter : public EditorSyntaxHighlighter {
    GDCLASS(CinchHighlighter, EditorSyntaxHighlighter);
    struct State {
        int depth = 0;
        char32_t quote = 0;
        bool escape = false;
    };
    mutable std::map<int, State> states;
    static bool letter(char32_t c) {
        return c == '_' || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c >= 128;
    }
    static bool digit(char32_t c) { return c >= '0' && c <= '9'; }
    static bool keyword(const String &word) {
        static const std::unordered_set<std::string> words = {
            "module",   "where",     "data",   "record",   "codata",     "effect",  "signature",
            "instance", "implicit",  "import", "as",       "type",       "opaque",  "forall",
            "Sigma",    "using",     "in",     "let",      "letrec",     "if",      "then",
            "else",     "case",      "of",     "do",       "handle",     "with",    "rewrite",
            "prim",     "law",       "target", "justify",  "match",      "replace", "require",
            "upgrade",  "contract",  "to",     "generate", "quoteDecls", "foreign", "export",
            "ccall",    "component", "proof",  "spec",     "stage",      "corec",   "macro",
            "infix",    "infixl",    "infixr", "erased",   "own",        "return",  "resume"};
        return words.count(word.utf8().get_data()) != 0;
    }
    Dictionary scan(const String &line, State &state, bool paint) const {
        Dictionary out;
        Color normal = get_text_edit()->get_theme_color("font_color");
        const Color comment(0.5, 0.55, 0.6), str(1.0, 0.85, 0.6), kw(1.0, 0.44, 0.55),
            type(0.55, 0.85, 0.8), number(0.65, 0.75, 1.0), op(0.75, 0.8, 0.9);
        auto color = [&](int start, int end, Color c) {
            if (!paint || end <= start)
                return;
            Dictionary a;
            a["color"] = c;
            out[start] = a;
            Dictionary b;
            b["color"] = normal;
            out[end] = b;
        };
        int n = line.length();
        int i = 0;
        while (i < n) {
            int start = i;
            char32_t c = line[i], next = i + 1 < n ? line[i + 1] : 0;
            if (state.depth) {
                while (i < n) {
                    if (i + 1 < n && line[i] == '{' && line[i + 1] == '-') {
                        ++state.depth;
                        i += 2;
                    } else if (i + 1 < n && line[i] == '-' && line[i + 1] == '}') {
                        --state.depth;
                        i += 2;
                        if (!state.depth)
                            break;
                    } else
                        ++i;
                }
                color(start, i, comment);
                continue;
            }
            if (state.quote) {
                while (i < n) {
                    char32_t ch = line[i++];
                    if (state.escape) {
                        state.escape = false;
                        continue;
                    }
                    if (ch == '\\') {
                        state.escape = true;
                        continue;
                    }
                    if (ch == state.quote) {
                        state.quote = 0;
                        break;
                    }
                }
                color(start, i, str);
                continue;
            }
            if (c == '-' && next == '-') {
                color(i, n, comment);
                break;
            }
            if (c == '{' && next == '-') {
                state.depth = 1;
                i += 2;
                color(start, i, comment);
                while (i < n) {
                    if (i + 1 < n && line[i] == '{' && line[i + 1] == '-') {
                        ++state.depth;
                        i += 2;
                    } else if (i + 1 < n && line[i] == '-' && line[i + 1] == '}') {
                        --state.depth;
                        i += 2;
                        if (!state.depth)
                            break;
                    } else
                        ++i;
                }
                color(start, i, comment);
                continue;
            }
            // Tick names ('symbol and ''Type) are not unterminated character literals.
            if (c == '\'' && next != '\\' && !(i + 2 < n && line[i + 2] == '\'')) {
                ++i;
                if (i < n && line[i] == '\'')
                    ++i;
                while (i < n && (letter(line[i]) || digit(line[i]) || line[i] == '\''))
                    ++i;
                color(start, i, type);
                continue;
            }
            if (c == '"' || c == '\'') {
                state.quote = c;
                ++i;
                while (i < n) {
                    char32_t ch = line[i++];
                    if (state.escape) {
                        state.escape = false;
                        continue;
                    }
                    if (ch == '\\') {
                        state.escape = true;
                        continue;
                    }
                    if (ch == state.quote) {
                        state.quote = 0;
                        break;
                    }
                }
                color(start, i, str);
                continue;
            }
            if (letter(c)) {
                ++i;
                while (i < n && (letter(line[i]) || digit(line[i]) || line[i] == '\'' ||
                                 line[i] == '!' || line[i] == '?' ||
                                 (line[i] == '-' && i + 1 < n &&
                                  (letter(line[i + 1]) || digit(line[i + 1])))))
                    ++i;
                String word = line.substr(start, i - start);
                if (keyword(word))
                    color(start, i, kw);
                else if (c >= 'A' && c <= 'Z')
                    color(start, i, type);
                continue;
            }
            if (digit(c)) {
                ++i;
                while (i < n &&
                       (digit(line[i]) || (line[i] >= 'a' && line[i] <= 'f') ||
                        (line[i] >= 'A' && line[i] <= 'F') || line[i] == 'x' || line[i] == 'X' ||
                        line[i] == '_' || (line[i] == '.' && i + 1 < n && digit(line[i + 1])) ||
                        ((line[i] == '+' || line[i] == '-') && i > start &&
                         (line[i - 1] == 'e' || line[i - 1] == 'E'))))
                    ++i;
                color(start, i, number);
                continue;
            }
            if (c != ' ' && c != '\t')
                color(i, i + 1, op);
            ++i;
        }
        // Cinch strings and character literals cannot span source lines.
        state.quote = 0;
        state.escape = false;
        return out;
    }

  protected:
    static void _bind_methods() {}

  public:
    String _get_name() const override { return "Cinch"; }
    PackedStringArray _get_supported_languages() const override { return {"Cinch"}; }
    Ref<EditorSyntaxHighlighter> _create() const override {
        Ref<CinchHighlighter> h;
        h.instantiate();
        return h;
    }
    void _clear_highlighting_cache() override { states.clear(); }
    void _update_cache() override {
        states.clear();
        // Cinch layout rejects tabs. Keep Godot's per-editor save conversion
        // and typing behavior compatible without changing global editor preferences.
        if (auto *code = Object::cast_to<CodeEdit>(get_text_edit())) {
            code->set_indent_using_spaces(true);
            code->set_indent_size(2);
        }
    }
    Dictionary _get_line_syntax_highlighting(int32_t line) const override {
        auto *edit = get_text_edit();
        if (!edit || line < 0 || line >= edit->get_line_count())
            return {};
        State s;
        int start = 0;
        auto it = states.lower_bound(line);
        if (it != states.begin()) {
            --it;
            start = it->first + 1;
            s = it->second;
        }
        for (int i = start; i < line; ++i) {
            scan(edit->get_line(i), s, false);
            states[i] = s;
        }
        Dictionary result = scan(edit->get_line(line), s, true);
        states[line] = s;
        return result;
    }
};
class CinchEditorPlugin : public EditorPlugin {
    GDCLASS(CinchEditorPlugin, EditorPlugin);
    Ref<CinchHighlighter> highlighter;

  protected:
    static void _bind_methods() {}

  public:
    void _enter_tree() override {
        highlighter.instantiate();
        auto *editor = EditorInterface::get_singleton();
        if (editor && editor->get_script_editor())
            editor->get_script_editor()->register_syntax_highlighter(highlighter);
    }
    void _exit_tree() override {
        auto *editor = EditorInterface::get_singleton();
        if (editor && editor->get_script_editor() && highlighter.is_valid())
            editor->get_script_editor()->unregister_syntax_highlighter(highlighter);
        highlighter.unref();
    }
};
static void initialize_editor() {
    ClassDB::register_class<CinchHighlighter>();
    ClassDB::register_class<CinchEditorPlugin>();
    EditorPlugins::add_by_type<CinchEditorPlugin>();
}
static void terminate_editor() { EditorPlugins::remove_by_type<CinchEditorPlugin>(); }
} // namespace cinch_scripts
