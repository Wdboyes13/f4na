#include <eval.h>
#include <unistd.h>
#include <iostream>

static Value c_println(std::vector<Value> args) {
    for (const auto& arg : args) {
        std::cout << arg;
    }
    std::cout << "\n";
    return 0;
}

static Value c_print(std::vector<Value> args) {
    for (const auto& arg : args) {
        std::cout << arg;
    }
    return 0;
}

static Value c_input(std::vector<Value> args) {
    std::cout << args[0];
    std::string buf;
    std::getline(std::cin, buf);
    return buf;
}

static Value c_syscall(std::vector<Value> args) {
    if (args.empty()) {
        throw std::runtime_error("syscall requires at least 1 argument");
    }

    long num = args[0].as_int();
    long ret;

    auto arg = [&](int n) -> long long {
        if (auto s = std::get_if<std::string>(&args[n].data)) {
            return reinterpret_cast<long long>(s->c_str());
        } else {
            return std::get<long long>(args[n].data);
        }
    };

    switch (args.size()) {
        case 1:
            ret = syscall(num);
            break;
        case 2:
            ret = syscall(num, arg(1));
            break;
        case 3:
            ret = syscall(num, arg(1), arg(2));
            break;
        case 4:
            ret = syscall(num, arg(1), arg(2), arg(3));
            break;
        case 5:
            ret = syscall(num, arg(1), arg(2), arg(3), arg(4));
            break;
        case 6:
            ret = syscall(num, arg(1), arg(2), arg(3), arg(4), arg(5));
            break;
        case 7:
            ret = syscall(num, arg(1), arg(2), arg(3), arg(4), arg(5), arg(6));
            break;
        default:
            throw std::runtime_error("syscall takes at most 7 arguments");
    }

    return (long long)ret;
}

static Value c_strlen(std::vector<Value> args) {
    if (args[0].type() == Type::STRING) {
        return (long long)std::get<std::string>(args[0].data).size();
    } else {
        return 0;
    }
}

static Value c_typeof(std::vector<Value> args) {
    switch (args[0].type()) {
        case Type::BOOL: {
            return "bool";
        }
        case Type::FLOAT: {
            return "float";
        }
        case Type::STRING: {
            return "string";
        }
        case Type::INT: {
            return "int";
        }
    }
}

static Value c_platformid(std::vector<Value> args) {
#if defined(__APPLE__)
    return "darwin";
#elif defined(__linux__)
    return "linux";
#elif defined(_WIN32) || defined(_WIN64)
    return "windows";
#endif
}

static Value c_sin(std::vector<Value> args) { return std::sin(args[0].as_float()); }
static Value c_cos(std::vector<Value> args) { return std::cos(args[0].as_float()); }
static Value c_tan(std::vector<Value> args) { return std::tan(args[0].as_float()); }
static Value c_abs(std::vector<Value> args) { return std::abs(args[0].as_float()); }
static Value c_sqrt(std::vector<Value> args) { return std::sqrt(args[0].as_float()); }
static Value c_log(std::vector<Value> args) { return std::log(args[0].as_float()); }
static Value c_log10(std::vector<Value> args) { return std::log10(args[0].as_float()); }

Environment* build_env() {
    auto env = new Environment();
    env->vars = {};
    env->ufnlut = {};

    env->bfnlut = {
        { "println", { -1, c_println } },
        { "print", { -1, c_print } },
        { "input", { 1, c_input } },
        { "syscall", { -1, c_syscall } },
        { "strlen", { 1, c_strlen } },
        { "typeof", { 1, c_typeof } },
        { "platformid", { 0, c_platformid } },
        { "sin", { 1, c_sin } },
        { "cos", { 1, c_cos } },
        { "tan", { 1, c_tan } },
        { "abs", { 1, c_abs } },
        { "sqrt", { 1, c_sqrt } },
        { "log", { 1, c_log } },
        { "log10", { 1, c_log10 } }
    };

    auto pbtns = [&](std::vector<Value> args) -> Value {
        std::cout << "name | nargs\n";
        for (const auto& bfn : env->bfnlut) {
            std::cout << bfn.first << " | " << bfn.second.nargs << "\n";
        }
        return 0;
    };

    env->bfnlut.insert({ "print_builtins", { 0, pbtns } });

    return env;
}