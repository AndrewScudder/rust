# 🔧 macOS App Fix Guide

## 🚨 **"App is Damaged" Error - Complete Fix**

If you're getting the "TimeCard is damaged and can't be opened" error, here's how to fix it:

### 🛠️ **Method 1: Terminal Fix (Recommended)**

**Open Terminal on your Mac and run:**

```bash
# Navigate to where you downloaded TimeCard.app
cd ~/Downloads  # or wherever you saved it

# Remove all quarantine attributes
xattr -cr TimeCard.app

# Self-sign the app
codesign --force --deep --sign - TimeCard.app

# Make sure it's executable
chmod +x TimeCard.app/Contents/MacOS/TimeCard

# Try opening it
open TimeCard.app
```

### 🛠️ **Method 2: Right-Click Method**

1. **Right-click** (or Control+click) `TimeCard.app`
2. Select **"Open"** from the context menu
3. Click **"Open"** in the dialog that appears
4. The app will now work normally

### 🛠️ **Method 3: System Preferences**

1. Go to **System Preferences** → **Security & Privacy**
2. Click the **"General"** tab
3. Look for a message about `TimeCard.app`
4. Click **"Allow Anyway"**

### 🛠️ **Method 4: Direct Execution**

**In Terminal:**
```bash
# Navigate to the app
cd /path/to/TimeCard.app/Contents/MacOS

# Run directly
./TimeCard gui
```

### 🛠️ **Method 5: Move to Applications**

```bash
# Move to Applications folder
sudo mv TimeCard.app /Applications/

# Remove quarantine from Applications
xattr -cr /Applications/TimeCard.app

# Open from Applications
open /Applications/TimeCard.app
```

## 🔍 **Why This Happens**

### **Common Causes:**
1. **Gatekeeper**: macOS blocks unsigned apps from internet
2. **Quarantine**: Downloads are marked as potentially unsafe
3. **Missing Permissions**: Executable permissions not set correctly
4. **Corrupted Download**: Incomplete file transfer

### **Solutions:**
- ✅ **Remove quarantine attributes**
- ✅ **Self-sign the app**
- ✅ **Set proper permissions**
- ✅ **Use right-click method**

## 🎯 **Quick Fix Commands**

**Copy and paste this entire block into Terminal:**

```bash
# Navigate to Downloads (adjust path if needed)
cd ~/Downloads

# Fix the app
xattr -cr TimeCard.app
codesign --force --deep --sign - TimeCard.app
chmod +x TimeCard.app/Contents/MacOS/TimeCard

# Test it
open TimeCard.app
```

## ✅ **After Fixing**

Once fixed, TimeCard will:
- ✅ Run perfectly on Apple Silicon
- ✅ Show up in Applications folder
- ✅ Work from Spotlight search
- ✅ Launch normally
- ✅ Track time efficiently

## 🚀 **Alternative: Use CLI Version**

If the GUI app still has issues, you can use the command-line version:

```bash
# Download the binary (not .app)
# Then run:
chmod +x timecard
./timecard gui
```

## 🎯 **Summary**

**The app isn't actually damaged** - it's just macOS being protective. Use the terminal commands above to fix it!

**TimeCard is completely safe and will work perfectly once the security restrictions are removed.** 🍎✨
