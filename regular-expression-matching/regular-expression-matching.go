import (
    "regexp"
)

func isMatch(s string, p string) bool {
        match, _ := regexp.MatchString("^"+p+"$",s)
        return match
}