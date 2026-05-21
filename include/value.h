#pragma once
#include <cstring>
#include <iostream>
#include <optional>
#include <string>
#include <type_traits>
#include <variant>

struct Value;

template<typename T>
concept Coercable = std::is_floating_point_v<T> || std::is_integral_v<T> || std::is_same_v<T, bool>;

enum class Type { INT,
                  FLOAT,
                  BOOL,
                  STRING };

template<typename T>
struct Coerced {
    T l, r;
    bool is_int;
};

struct Value {
    using Data = std::variant<long long, double, bool, std::string>;
    Data data;

    Type type() const;

    long long as_int() const;
    double as_float() const;
    bool as_bool() const;
    bool is_string() const;
    void ensure_ntype(const std::optional<Value>& b, std::initializer_list<Type> types) const;

    template<typename T>
    Value(T value) : data(value) {}
    Value() : data(0LL) {}

    Value operator+(Value b);
    Value operator-(Value b);
    Value operator*(Value b);
    Value operator/(Value b);
    Value operator%(Value b);
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

    Value operator|(Value b);
    Value operator^(Value b);
    Value operator&(Value b);
    Value operator<<(Value b);
    Value operator>>(Value b);
    Value operator~();

    Value pow(Value b);
};

std::ostream& operator<<(std::ostream& os, const Value& v);

template<Coercable T>
inline Coerced<T> coerce(Value a, Value b) {
    bool both_int = a.type() == Type::INT && b.type() == Type::INT;

    if constexpr (std::is_floating_point_v<T>) {
        return Coerced<T>{ a.as_float(), b.as_float(), both_int };
    } else if constexpr (std::is_same_v<T, bool>) {
        return Coerced<T>{ a.as_bool(), b.as_bool(), both_int };
    } else if constexpr (std::is_integral_v<T>) {
        return Coerced<T>{ a.as_int(), b.as_int(), both_int };
    }
}