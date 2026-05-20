#pragma once
#include <cstring>
#include <iostream>
#include <optional>
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

    Type type() const;

    long long as_int() const;
    double as_float() const;
    bool as_bool() const;
    bool is_string() const;
    void ensure_nstr(const std::optional<Value>& b = std::nullopt) const;

    template<typename T>
    Value(T value) : data(value) {}
    Value() : data(0LL) {}

    Value operator+(Value b);
    Value operator-(Value b);
    Value operator*(Value b);
    Value operator/(Value b);
    Value operator%(Value b);
    Value operator^(Value exp);
    Value operator==(Value b);
    Value operator!=(Value b);
    Value operator<(Value b);
    Value operator>(Value b);
    Value operator<=(Value b);
    Value operator>=(Value b);
    Value operator&&(Value b);
    Value operator||(Value b);
    Value operator-();
    Value operator+();
    Value operator!();
};

std::ostream& operator<<(std::ostream& os, const Value& v);