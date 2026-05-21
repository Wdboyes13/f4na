#include <value.h>
#include <cmath>
#include <stdexcept>

Type Value::type() const {
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

long long Value::as_int() const {
    if (auto* v = std::get_if<double>(&data)) {
        return (long long)*v;
    }
    if (auto* v = std::get_if<bool>(&data)) {
        return (long long)*v;
    }
    return std::get<long long>(data);
}

double Value::as_float() const {
    if (auto* v = std::get_if<long long>(&data)) {
        return (double)*v;
    }
    if (auto* v = std::get_if<bool>(&data)) {
        return (double)*v;
    }
    return std::get<double>(data);
}

bool Value::as_bool() const {
    if (auto* v = std::get_if<long long>(&data)) {
        return (bool)*v;
    }
    if (auto* v = std::get_if<double>(&data)) {
        return (bool)*v;
    }
    return std::get<bool>(data);
}

bool Value::is_string() const { return std::holds_alternative<std::string>(data); }

void Value::ensure_ntype(const std::optional<Value>& b, std::initializer_list<Type> types) const {
    for (auto& t : types) {
        if (type() == t || b->type() == t) {
            throw std::runtime_error("this operation does not accept this type");
        }
    }
}

Value Value::operator+(Value b) {
    ensure_ntype(b, { Type::STRING });
    auto [l, r, is_int] = coerce<double>(*this, b);
    if (is_int) {
        return Value((long long)(l + r));
    }
    return Value(l + r);
}

Value Value::operator-(Value b) {
    ensure_ntype(b, { Type::STRING });
    auto [l, r, is_int] = coerce<double>(*this, b);
    if (is_int) {
        return Value((long long)l - r);
    }
    return Value(l - r);
}

Value Value::operator*(Value b) {
    ensure_ntype(b, { Type::STRING });
    auto [l, r, is_int] = coerce<double>(*this, b);
    if (is_int) {
        return Value((long long)l * r);
    }
    return Value(l * r);
}

Value Value::operator/(Value b) {
    ensure_ntype(b, { Type::STRING });
    auto [l, r, _] = coerce<double>(*this, b);
    if (r == 0.0) {
        throw std::runtime_error("division by zero");
    }
    return Value{ l / r };
}

Value Value::operator%(Value b) {
    ensure_ntype(b, { Type::STRING });
    if (!(this->type() == Type::INT) || !(b.type() == Type::INT)) {
        throw std::runtime_error("% requires integers");
    }

    if (std::get<long long>(b.data) == 0) {
        throw std::runtime_error("modulo by zero");
    }

    return Value(std::get<long long>(data) % std::get<long long>(b.data));
}

Value Value::pow(Value b) {
    ensure_ntype(b, { Type::STRING });
    auto [l, r, is_int] = coerce<double>(*this, b);
    if (is_int) {
        return Value{ (long long)std::pow(l, r) };
    }
    return Value{ std::pow(l, r) };
}

Value Value::operator==(Value b) {
    if (this->is_string() && b.is_string()) {
        return std::get<std::string>(this->data) == std::get<std::string>(b.data);
    } else {
        throw std::runtime_error("cannot compare string and different type");
    }

    auto [l, r, _] = coerce<double>(*this, b);
    return Value{ l == r };
}

Value Value::operator!=(Value b) {
    if (this->is_string() && b.is_string()) {
        return std::get<std::string>(this->data) != std::get<std::string>(b.data);
    } else {
        throw std::runtime_error("cannot compare string and different type");
    }

    auto [l, r, _] = coerce<double>(*this, b);
    return Value{ l != r };
}

Value Value::operator<(Value b) {
    ensure_ntype(b, { Type::STRING });
    auto [l, r, _] = coerce<double>(*this, b);
    return Value{ l < r };
}

Value Value::operator>(Value b) {
    ensure_ntype(b, { Type::STRING });
    auto [l, r, _] = coerce<double>(*this, b);
    return Value{ l > r };
}

Value Value::operator<=(Value b) {
    ensure_ntype(b, { Type::STRING });
    auto [l, r, _] = coerce<double>(*this, b);
    return Value{ l <= r };
}

Value Value::operator>=(Value b) {
    ensure_ntype(b, { Type::STRING });
    auto [l, r, _] = coerce<double>(*this, b);
    return Value{ l >= r };
}

Value Value::operator&&(Value b) {
    ensure_ntype(b, { Type::STRING });
    return Value{ this->as_bool() && b.as_bool() };
}

Value Value::operator||(Value b) {
    ensure_ntype(b, { Type::STRING });
    return Value{ this->as_bool() || b.as_bool() };
}

Value Value::operator-() {
    ensure_ntype(std::nullopt, { Type::STRING, Type::BOOL });
    return Value(-as_float());
}

Value Value::operator+() {
    ensure_ntype(std::nullopt, { Type::STRING, Type::BOOL });
    return Value(+as_float());
}

Value Value::operator!() {
    ensure_ntype(std::nullopt, { Type::STRING });
    return Value(!as_int());
}

Value Value::operator|(Value b) {
    ensure_ntype(std::nullopt, { Type::FLOAT, Type::STRING });
    auto [l, r, _] = coerce<int>(*this, b);
    return l | r;
}

Value Value::operator^(Value b) {
    ensure_ntype(std::nullopt, { Type::FLOAT, Type::STRING });
    auto [l, r, _] = coerce<int>(*this, b);
    return l ^ r;
}

Value Value::operator&(Value b) {
    ensure_ntype(std::nullopt, { Type::FLOAT, Type::STRING });
    auto [l, r, _] = coerce<int>(*this, b);
    return l & r;
}

Value Value::operator<<(Value b) {
    ensure_ntype(std::nullopt, { Type::FLOAT, Type::STRING });
    auto [l, r, _] = coerce<int>(*this, b);
    return l << r;
}

Value Value::operator>>(Value b) {
    ensure_ntype(std::nullopt, { Type::FLOAT, Type::STRING });
    auto [l, r, _] = coerce<int>(*this, b);
    return l >> r;
}

Value Value::operator~() {
    ensure_ntype(std::nullopt, { Type::FLOAT, Type::STRING });
    return ~this->as_int();
}

std::ostream& operator<<(std::ostream& os, const Value& v) {
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
