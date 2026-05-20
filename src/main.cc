#include <ast.h>
#include <eval.h>
#include <tokenizer.h>
#include <cstdio>
#include <fstream>

int main(int ac, char** av) {
    if (ac < 2) {
        fprintf(stderr, "not enough arguments\n");
        return 1;
    }

    std::ifstream file(av[1]);
    if (!file) {
        fprintf(stderr, "failed to open file\n");
        return 1;
    }

    std::string progbuf{
        std::istreambuf_iterator<char>(file),
        std::istreambuf_iterator<char>()
    };

    file.close();

    try {
        auto tokens = tokenize(progbuf);
        auto ast = parse(tokens);
        eval(ast);
    } catch (std::exception& e) {
        std::cout << e.what() << "\n";
        return 1;
    }

    return 0;
}