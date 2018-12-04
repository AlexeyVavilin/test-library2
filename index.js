
import { NativeModules } from 'react-native';
import { RNTestLibrary2Module } from 'NativeModules';

const { RNTestLibrary2 } = NativeModules;

export default RNTestLibrary2;

module.exports = function factorize(Challenge) {
	RNTestLibrary2Module.factorize(Challenge)
};
