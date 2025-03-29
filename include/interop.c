#pragma comment(lib, "SHELL32.LIB")
#include <wchar.h>
#include <Windows.h>

int compute_program_length(const wchar_t* commandline)
{
  int i = 0;

  if (commandline[0] == L'"') {
    // Wait till end of string
    i++;
  }

  for (;;) {
    i++;
    wchar_t c = commandline[i];

    // String already terminated
    if (c == 0)
      return i - 1;
    // End of string
    else if (c == L'"')
      return i;
  }
}

BOOL is_windows_app(const wchar_t* path) {
    SHFILEINFOW fileInfo;
    const BOOL is_windows_app = HIWORD(SHGetFileInfoW(path, 0, &fileInfo, sizeof(fileInfo), SHGFI_EXETYPE));
}
