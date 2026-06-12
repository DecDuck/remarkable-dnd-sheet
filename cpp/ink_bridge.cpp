#include "inkcanvas.h"

#include <qqml.h>

// Registers the C++ ink rendering type so QML can instantiate it via
// `import InkTools 1.0`. Called from Rust main() before the QML is loaded.
extern "C" void register_ink_types()
{
    qmlRegisterType<InkCanvas>("InkTools", 1, 0, "InkCanvas");
}
