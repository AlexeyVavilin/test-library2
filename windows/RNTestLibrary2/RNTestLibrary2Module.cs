using ReactNative.Bridge;
using System;
using System.Collections.Generic;
using Windows.ApplicationModel.Core;
using Windows.UI.Core;

namespace Test.Library2.RNTestLibrary2
{
    /// <summary>
    /// A module that allows JS to share data.
    /// </summary>
    class RNTestLibrary2Module : NativeModuleBase
    {
        /// <summary>
        /// Instantiates the <see cref="RNTestLibrary2Module"/>.
        /// </summary>
        internal RNTestLibrary2Module()
        {

        }

        /// <summary>
        /// The name of the native module.
        /// </summary>
        public override string Name
        {
            get
            {
                return "RNTestLibrary2";
            }
        }
    }
}
