#include "re2/re2.h"
#include <memory>

using namespace re2;

extern "C" bool validate(const char* pattern) {
    const auto opts = RE2::Options(RE2::CannedOptions::Quiet);
    const auto re2 = std::make_unique<RE2>(pattern, opts);
    return re2->ok();
}

extern "C" int error_code(const char* pattern) {
    const auto opts = RE2::Options(RE2::CannedOptions::Quiet);
    const auto re2 = std::make_unique<RE2>(pattern, opts);
    return re2->error_code();
}