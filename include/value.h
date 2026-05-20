#pragma once
#include <cmath>
#include <cstring>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>
#include <variant>

struct Value;

enum class Type { INT,
                  FLOAT,
                  BOOL,
                  STRING };

struct Coerced {
    double l, r;
    bool is_int;
};

Coerced coerce(Value a, Value b);

struct Value {
    using Data = std::variant<long long, double, bool, std::string>;
    Data data;

    Type type() const {
        switch (data.index()) {
            case 0:
                return Type::INT;
            case 1:
                return Type::FLOAT;
            case 2:
                return Type::BOOL;
            case 3:
                return Type::STRING;
            default:
                throw std::runtime_error("invalid type");
        }
    }

    long long as_int() const {
        if (auto* v = std::get_if<double>(&data)) {
            return (long long)*v;
        }
        if (auto* v = std::get_if<bool>(&data)) {
            return (long long)*v;
        }
        return std::get<long long>(data);
    }

    double as_float() const {
        if (auto* v = std::get_if<long long>(&data)) {
            return (double)*v;
        }
        if (auto* v = std::get_if<bool>(&data)) {
            return (double)*v;
        }
        return std::get<double>(data);
    }

    bool as_bool() const {
        if (auto* v = std::get_if<long long>(&data)) {
            return (bool)*v;
        }
        if (auto* v = std::get_if<double>(&data)) {
            return (bool)*v;
        }
        return std::get<bool>(data);
    }

    bool is_string() const { return std::holds_alternative<std::string>(data); }

    void ensure_nstr(const std::optional<Value>& b = std::nullopt) const {
        if (is_string()) {
            throw std::runtime_error("this operation doesn't support strings");
        }
        if (b.has_value() && b->is_string()) {
            throw std::runtime_error("this operation doesn't support strings");
        }
    }

    template<typename T>
    Value(T value) : data(value) {}

    Value() : data(0LL) {}

    Value operator+(Value b) {
        ensure_nstr(b);
        auto [l, r, is_int] = coerce(*this, b);
        if (is_int) {
            return Value((long long)(l + r));
        }
        return Value(l + r);
    }

    Value operator-(Value b) {
        ensure_nstr(b);
        auto [l, r, is_int] = coerce(*this, b);
        if (is_int) {
            return Value((long long)l - r);
        }
        return Value(l - r);
    }

    Value operator*(Value b) {
        ensure_nstr(b);
        auto [l, r, is_int] = coerce(*this, b);
        if (is_int) {
            return Value((long long)l * r);
        }
        return Value(l * r);
    }

    Value operator/(Value b) {
        ensure_nstr(b);
        auto [l, r, _] = coerce(*this, b);
        if (r == 0.0) {
            throw std::runtime_error("division by zero");
        }
        return Value{ l / r };
    }

    Value operator%(Value b) {
        ensure_nstr(b);
        if (!(this->type() == Type::INT) || !(b.type() == Type::INT)) {
            throw std::runtime_error("% requires integers");
        }

        if (std::get<long long>(b.data) == 0) {
            throw std::runtime_error("modulo by zero");
        }

        return Value(std::get<long long>(data) % std::get<long long>(b.data));
    }

    Value operator^(Value exp) {
        ensure_nstr(exp);
        auto [l, r, is_int] = coerce(*this, exp);
        if (is_int) {
            return Value{ (long long)std::pow(l, r) };
        }
        return Value{ std::pow(l, r) };
    }

    Value operator==(Value b) {
        ensure_nstr(b);
        auto [l, r, _] = coerce(*this, b);
        return Value{ l == r };
    }

    Value operator!=(Value b) {
        ensure_nstr(b);
        auto [l, r, _] = coerce(*this, b);
        return Value{ l != r };
    }

    Value operator<(Value b) {
        ensure_nstr(b);
        auto [l, r, _] = coerce(*this, b);
        return Value{ l < r };
    }

    Value operator>(Value b) {
        ensure_nstr(b);
        auto [l, r, _] = coerce(*this, b);
        return Value{ l > r };
    }

    Value operator<=(Value b) {
        ensure_nstr(b);
        auto [l, r, _] = coerce(*this, b);
        return Value{ l <= r };
    }

    Value operator>=(Value b) {
        ensure_nstr(b);
        auto [l, r, _] = coerce(*this, b);
        return Value{ l >= r };
    }

    Value operator&&(Value b) {
        ensure_nstr(b);
        return Value{ this->as_bool() && b.as_bool() };
    }

    Value operator||(Value b) {
        ensure_nstr(b);
        return Value{ this->as_bool() || b.as_bool() };
    }

    Value operator-() {
        ensure_nstr();
        if (type() == Type::BOOL) {
            throw std::runtime_error("cannot negate a bool");
        }
        return Value(-as_float());
    }

    Value operator+() {
        ensure_nstr();
        if (type() == Type::BOOL) {
            throw std::runtime_error("cannot negate a bool");
        }
        return Value(+as_float());
    }

    Value operator!() {
        ensure_nstr();
        return Value(!as_int());
    }
};

inline std::ostream& operator<<(std::ostream& os, const Value& v) {
    switch (v.type()) {
        case Type::INT:
            os << v.as_int();
            break;
        case Type::FLOAT:
            os << v.as_float();
            break;
        case Type::BOOL:
            os << (v.as_bool() ? "true" : "false");
            break;
        case Type::STRING:
            os << std::get<std::string>(v.data);
            break;
    }
    return os;
}

inline Coerced coerce(Value a, Value b) {
    bool both_int = a.type() == Type::INT && b.type() == Type::INT;
    return { a.as_float(), b.as_float(), both_int };
}