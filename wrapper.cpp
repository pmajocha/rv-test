#include "re2/re2.h"
#include <memory>

using namespace re2;

struct MatchResult {
    int error_code;
    bool is_match;
};

extern "C" bool validate(const char* pattern) {
    const auto opts = RE2::Options(RE2::CannedOptions::Quiet);
    const auto re2 = RE2(pattern, opts);
    return re2.ok();
}

extern "C" int error_code(const char* pattern) {
    const auto opts = RE2::Options(RE2::CannedOptions::Quiet);
    const auto re2 = RE2(pattern, opts);
    return re2.error_code();
}

extern "C" MatchResult is_match(const char* pattern, const char* text) {
    const auto opts = RE2::Options(RE2::CannedOptions::Quiet);
    const auto re2 = RE2(pattern, opts);

    if (re2.ok()) {
        return MatchResult { 0, RE2::PartialMatch(text, re2) };
    } else {
        return MatchResult { re2.error_code(), false };
    }
}