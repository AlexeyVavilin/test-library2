
# react-native-test-library2

## Getting started

`$ npm install react-native-test-library2 --save`

### Mostly automatic installation

`$ react-native link react-native-test-library2`

### Manual installation


#### iOS

1. In XCode, in the project navigator, right click `Libraries` ➜ `Add Files to [your project's name]`
2. Go to `node_modules` ➜ `react-native-test-library2` and add `RNTestLibrary2.xcodeproj`
3. In XCode, in the project navigator, select your project. Add `libRNTestLibrary2.a` to your project's `Build Phases` ➜ `Link Binary With Libraries`
4. Run your project (`Cmd+R`)<

#### Android

1. Open up `android/app/src/main/java/[...]/MainActivity.java`
  - Add `import com.reactlibrary.RNTestLibrary2Package;` to the imports at the top of the file
  - Add `new RNTestLibrary2Package()` to the list returned by the `getPackages()` method
2. Append the following lines to `android/settings.gradle`:
  	```
  	include ':react-native-test-library2'
  	project(':react-native-test-library2').projectDir = new File(rootProject.projectDir, 	'../node_modules/react-native-test-library2/android')
  	```
3. Insert the following lines inside the dependencies block in `android/app/build.gradle`:
  	```
      compile project(':react-native-test-library2')
  	```

#### Windows
[Read it! :D](https://github.com/ReactWindows/react-native)

1. In Visual Studio add the `RNTestLibrary2.sln` in `node_modules/react-native-test-library2/windows/RNTestLibrary2.sln` folder to their solution, reference from their app.
2. Open up your `MainPage.cs` app
  - Add `using Test.Library2.RNTestLibrary2;` to the usings at the top of the file
  - Add `new RNTestLibrary2Package()` to the `List<IReactPackage>` returned by the `Packages` method


## Usage
```javascript
import RNTestLibrary2 from 'react-native-test-library2';

// TODO: What to do with the module?
RNTestLibrary2;
```
  