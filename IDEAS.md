You can launch iOS emulators (simulators) directly from the terminal using xcrun and simctl, tools provided by Xcode. Here’s how:

1. List Available Simulators

Run the following command to list all available iOS simulators:

xcrun simctl list devices

This will display all available devices and their states (e.g., Booted or Shutdown).

2. Boot a Specific Simulator

To boot a specific simulator, you need its UDID (from the list above). Run:

xcrun simctl boot <UDID>

For example:

xcrun simctl boot F2D99A30-872C-4F8F-B674-2BEB1F6E82E4

3. Open the Simulator App

After booting, open the Simulator app:

open -a Simulator

4. Combine Commands

To simplify, you can combine the commands into one:

xcrun simctl boot <UDID> && open -a Simulator

5. Use a Shortcut for a Named Device

If you know the device’s name (e.g., “iPhone 14”), you can skip looking up the UDID by using:

xcrun simctl boot "iPhone 14" && open -a Simulator

6. Shutdown All Simulators

To shut down all running simulators:

xcrun simctl shutdown all

This allows you to manage and launch your iOS simulators directly from the terminal.