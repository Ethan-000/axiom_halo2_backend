use crate::pse_halo2::circuit_translator::NoirHalo2Translator;
use acvm::acir::circuit::Circuit;
use acvm::acir::native_types::WitnessMap;
use pse_halo2wrong::curves::bn256::Fr;

const add_circuit: Vec<u8> = vec![205, 151, 203, 78, 194, 64, 24, 133, 165, 148, 75, 75, 75, 216, 186, 235, 35, 116, 122, 129, 153, 157, 224, 45, 110, 76, 244, 13, 160, 14, 177, 17, 33, 33, 133, 184, 237, 27, 244, 162, 143, 160, 81, 87, 62, 130, 175, 97, 226, 195, 104, 140, 36, 176, 153, 5, 115, 8, 51, 171, 38, 147, 156, 156, 255, 155, 51, 255, 63, 125, 236, 124, 31, 28, 166, 31, 131, 201, 48, 186, 27, 204, 30, 206, 22, 211, 232, 120, 56, 153, 164, 207, 215, 253, 203, 243, 211, 188, 168, 56, 130, 77, 77, 180, 89, 117, 210, 183, 147, 120, 206, 163, 36, 94, 242, 244, 245, 106, 49, 75, 98, 62, 77, 158, 202, 172, 40, 190, 142, 92, 185, 69, 42, 0, 13, 77, 90, 194, 45, 179, 108, 107, 17, 178, 250, 168, 233, 159, 2, 142, 186, 147, 190, 247, 231, 113, 114, 123, 207, 147, 56, 42, 243, 82, 190, 240, 90, 45, 255, 197, 231, 187, 221, 32, 224, 61, 143, 19, 159, 12, 93, 143, 141, 104, 232, 6, 225, 168, 75, 9, 37, 33, 13, 111, 60, 234, 251, 156, 6, 180, 199, 70, 172, 231, 50, 18, 248, 156, 140, 67, 230, 143, 87, 182, 229, 241, 109, 84, 150, 149, 0, 83, 21, 128, 134, 6, 200, 86, 103, 23, 112, 100, 77, 233, 5, 32, 172, 0, 192, 29, 21, 147, 83, 5, 1, 150, 245, 81, 7, 192, 89, 107, 188, 47, 23, 211, 37, 159, 39, 69, 189, 1, 111, 35, 245, 6, 162, 141, 52, 193, 97, 128, 84, 214, 204, 21, 57, 201, 141, 152, 231, 128, 136, 54, 177, 241, 90, 159, 235, 0, 123, 84, 87, 100, 38, 155, 134, 104, 38, 27, 206, 14, 25, 16, 115, 191, 12, 86, 43, 176, 90, 34, 6, 45, 13, 126, 237, 44, 11, 113, 237, 44, 21, 167, 139, 9, 200, 69, 171, 0, 156, 169, 74, 112, 248, 248, 111, 49, 29, 0, 199, 0, 188, 109, 76, 21, 147, 99, 108, 175, 225, 253, 107, 16, 27, 224, 163, 45, 205, 151, 137, 154, 166, 52, 38, 121, 123, 144, 193, 97, 183, 69, 77, 179, 141, 255, 153, 179, 109, 4, 62, 91, 197, 183, 10, 192, 84, 94, 205, 126, 0];
const add_witnesses: Vec<u8> = vec![165, 210, 199, 9, 64, 33, 0, 4, 81, 126, 14, 6, 44, 69, 81, 208, 163, 197, 136, 237, 88, 159, 221, 8, 150, 224, 76, 1, 143, 61, 108, 219, 122, 182, 44, 191, 99, 34, 28, 152, 136, 39, 39, 46, 76, 216, 155, 19, 15, 39, 94, 78, 124, 148, 240, 233, 231, 43, 4, 39, 36, 39, 20, 39, 244, 58, 81, 103, 197, 25, 126, 240, 1];
const bit_and_circuit: Vec<u8> = vec![237, 157, 89, 79, 20, 105, 24, 133, 7, 25, 100, 17, 23, 68, 69, 64, 4, 68, 64, 68, 180, 170, 23, 186, 154, 125, 223, 247, 53, 174, 9, 98, 163, 56, 142, 70, 4, 196, 53, 225, 7, 168, 221, 141, 154, 185, 240, 198, 93, 188, 154, 27, 247, 13, 247, 37, 245, 7, 38, 153, 139, 249, 13, 115, 49, 215, 131, 201, 204, 133, 201, 164, 51, 73, 61, 157, 156, 76, 198, 91, 147, 147, 58, 207, 119, 234, 123, 235, 125, 155, 250, 234, 178, 253, 219, 175, 223, 45, 204, 206, 55, 140, 79, 4, 70, 39, 199, 167, 3, 179, 247, 122, 167, 142, 77, 142, 7, 142, 78, 94, 153, 11, 134, 194, 191, 212, 24, 206, 254, 153, 49, 142, 37, 140, 185, 96, 208, 161, 136, 105, 24, 223, 199, 46, 204, 254, 92, 119, 100, 100, 244, 135, 186, 99, 51, 77, 83, 71, 71, 235, 71, 142, 28, 153, 189, 221, 87, 219, 213, 220, 24, 10, 199, 38, 68, 248, 207, 239, 237, 63, 102, 239, 215, 78, 140, 79, 30, 250, 49, 48, 57, 62, 58, 23, 156, 91, 228, 226, 54, 74, 61, 158, 128, 207, 21, 48, 221, 230, 136, 225, 242, 239, 183, 188, 134, 199, 187, 191, 212, 50, 45, 211, 107, 121, 15, 184, 44, 183, 59, 96, 121, 44, 159, 127, 191, 223, 103, 248, 77, 143, 59, 96, 142, 121, 253, 238, 177, 191, 46, 41, 6, 96, 27, 27, 6, 184, 56, 95, 159, 104, 198, 103, 137, 72, 124, 150, 198, 69, 138, 79, 92, 164, 248, 44, 141, 70, 124, 150, 0, 108, 227, 128, 248, 44, 213, 142, 79, 172, 72, 124, 18, 226, 35, 197, 39, 62, 82, 124, 18, 162, 17, 159, 88, 128, 109, 60, 16, 159, 4, 237, 248, 196, 137, 196, 39, 41, 49, 82, 124, 18, 35, 197, 39, 41, 26, 241, 137, 3, 216, 38, 2, 241, 73, 98, 227, 51, 63, 112, 172, 35, 208, 55, 114, 96, 124, 102, 142, 201, 79, 188, 243, 235, 251, 201, 48, 93, 110, 143, 183, 212, 183, 228, 155, 101, 12, 205, 57, 191, 58, 195, 8, 1, 89, 48, 128, 53, 160, 157, 153, 38, 225, 204, 20, 116, 230, 114, 17, 206, 92, 130, 206, 220, 110, 194, 153, 91, 208, 153, 199, 67, 56, 243, 8, 58, 243, 122, 9, 103, 94, 65, 103, 165, 165, 132, 179, 82, 65, 103, 62, 31, 225, 204, 7, 59, 115, 244, 88, 18, 24, 251, 250, 111, 212, 44, 117, 172, 97, 153, 62, 0, 206, 78, 61, 56, 99, 166, 199, 177, 70, 192, 244, 2, 112, 118, 9, 194, 57, 224, 114, 174, 225, 119, 3, 112, 118, 71, 3, 142, 211, 29, 99, 23, 160, 177, 27, 128, 179, 71, 48, 57, 99, 38, 0, 103, 39, 0, 103, 175, 98, 114, 226, 137, 254, 2, 184, 142, 125, 138, 112, 246, 0, 26, 123, 1, 192, 251, 196, 155, 232, 68, 160, 137, 182, 252, 101, 229, 21, 149, 85, 213, 124, 19, 109, 89, 196, 227, 148, 37, 248, 160, 232, 247, 19, 206, 252, 130, 206, 202, 202, 8, 103, 101, 130, 206, 202, 203, 9, 103, 229, 130, 206, 42, 42, 8, 103, 21, 130, 206, 42, 43, 9, 103, 149, 130, 206, 170, 170, 8, 103, 85, 130, 206, 170, 171, 9, 103, 213, 138, 77, 116, 21, 208, 68, 87, 3, 112, 2, 138, 77, 116, 5, 208, 68, 87, 2, 112, 198, 20, 155, 232, 50, 160, 137, 46, 7, 224, 28, 84, 124, 218, 31, 3, 52, 14, 2, 112, 14, 41, 54, 209, 126, 0, 78, 0, 128, 51, 174, 152, 156, 68, 162, 191, 0, 174, 227, 176, 34, 156, 67, 128, 198, 56, 0, 248, 176, 224, 67, 138, 129, 180, 166, 199, 245, 156, 185, 76, 164, 53, 157, 208, 115, 230, 113, 33, 173, 233, 9, 61, 103, 150, 27, 105, 77, 39, 229, 156, 153, 134, 7, 105, 77, 167, 228, 156, 185, 12, 47, 210, 154, 78, 203, 57, 243, 24, 165, 72, 107, 122, 82, 206, 153, 101, 248, 144, 214, 116, 70, 177, 216, 159, 4, 52, 102, 0, 56, 167, 20, 225, 76, 1, 26, 211, 0, 156, 211, 138, 112, 78, 0, 26, 147, 0, 156, 51, 138, 112, 78, 3, 26, 103, 0, 56, 103, 21, 225, 76, 0, 26, 167, 0, 56, 231, 148, 250, 246, 191, 47, 106, 25, 0, 231, 56, 160, 113, 94, 49, 57, 103, 1, 141, 115, 192, 34, 157, 87, 76, 78, 60, 0, 103, 25, 112, 29, 201, 236, 143, 223, 119, 90, 143, 78, 7, 38, 38, 195, 201, 203, 241, 110, 62, 121, 57, 241, 92, 183, 66, 112, 78, 145, 188, 34, 36, 178, 146, 223, 196, 156, 248, 219, 133, 21, 209, 137, 87, 204, 74, 124, 17, 98, 86, 18, 139, 176, 74, 48, 94, 49, 171, 8, 103, 49, 116, 188, 194, 196, 219, 145, 128, 177, 20, 231, 191, 138, 253, 83, 68, 83, 86, 227, 11, 153, 178, 154, 88, 200, 84, 193, 136, 166, 164, 134, 152, 149, 196, 157, 57, 222, 16, 82, 137, 156, 175, 2, 52, 82, 181, 95, 53, 148, 120, 79, 222, 50, 140, 181, 107, 162, 122, 28, 192, 26, 5, 155, 30, 195, 72, 91, 23, 233, 141, 202, 117, 73, 248, 45, 158, 150, 70, 220, 226, 105, 138, 207, 242, 107, 128, 92, 172, 11, 3, 107, 170, 125, 135, 175, 229, 162, 239, 113, 34, 146, 190, 62, 82, 244, 215, 219, 191, 227, 217, 79, 79, 39, 178, 159, 174, 152, 253, 181, 64, 48, 214, 135, 129, 69, 85, 132, 67, 156, 113, 226, 120, 115, 249, 90, 211, 254, 63, 227, 228, 223, 96, 202, 204, 136, 106, 233, 207, 16, 41, 253, 89, 27, 34, 237, 127, 27, 248, 210, 159, 149, 69, 108, 127, 89, 138, 119, 120, 6, 144, 139, 13, 64, 233, 207, 210, 190, 195, 51, 69, 74, 127, 246, 198, 72, 209, 223, 24, 133, 210, 159, 157, 77, 100, 63, 91, 49, 251, 153, 64, 48, 54, 2, 165, 95, 18, 14, 113, 62, 85, 6, 80, 250, 51, 181, 55, 6, 137, 118, 120, 17, 83, 110, 78, 84, 75, 127, 142, 72, 233, 207, 219, 20, 105, 255, 219, 196, 151, 254, 188, 60, 98, 251, 203, 83, 188, 195, 115, 128, 92, 108, 2, 74, 127, 158, 246, 29, 158, 43, 82, 250, 243, 55, 71, 138, 254, 230, 40, 148, 254, 252, 124, 34, 251, 249, 138, 217, 207, 5, 130, 177, 25, 40, 253, 146, 112, 136, 113, 96, 14, 80, 250, 115, 181, 55, 6, 137, 118, 120, 17, 83, 97, 65, 84, 75, 127, 129, 72, 233, 47, 218, 18, 105, 255, 219, 194, 151, 254, 162, 34, 98, 251, 43, 82, 188, 195, 11, 128, 92, 108, 1, 74, 127, 145, 246, 29, 94, 40, 82, 250, 139, 183, 70, 138, 254, 214, 40, 148, 254, 226, 98, 34, 251, 197, 138, 217, 47, 4, 130, 177, 21, 40, 253, 146, 112, 136, 113, 96, 1, 80, 250, 11, 197, 143, 180, 1, 250, 225, 107, 53, 181, 117, 245, 13, 141, 77, 205, 45, 173, 109, 237, 252, 177, 54, 53, 53, 196, 29, 92, 35, 248, 183, 54, 181, 181, 132, 179, 90, 65, 103, 117, 117, 132, 179, 58, 65, 103, 245, 245, 132, 179, 122, 65, 103, 13, 13, 132, 179, 6, 65, 103, 141, 141, 132, 179, 70, 65, 103, 77, 77, 132, 179, 38, 65, 103, 205, 205, 132, 179, 102, 65, 103, 45, 45, 132, 179, 22, 65, 103, 173, 173, 132, 179, 86, 65, 103, 109, 109, 132, 179, 54, 65, 103, 237, 237, 132, 179, 118, 185, 163, 92, 70, 23, 23, 205, 169, 134, 181, 136, 7, 128, 99, 95, 208, 59, 232, 102, 113, 27, 114, 170, 17, 88, 188, 225, 9, 58, 23, 21, 143, 30, 107, 2, 142, 30, 107, 38, 232, 92, 82, 60, 123, 172, 1, 56, 123, 172, 145, 160, 19, 84, 60, 124, 172, 14, 56, 124, 172, 158, 160, 19, 82, 124, 23, 213, 14, 18, 34, 68, 209, 178, 195, 146, 124, 46, 18, 34, 151, 8, 62, 115, 138, 7, 180, 213, 18, 120, 46, 16, 120, 46, 75, 198, 7, 17, 185, 76, 240, 185, 162, 200, 39, 135, 152, 37, 18, 136, 175, 74, 166, 135, 120, 69, 206, 190, 66, 164, 231, 170, 248, 224, 28, 248, 53, 249, 90, 71, 103, 87, 119, 79, 111, 95, 255, 192, 224, 208, 48, 63, 56, 239, 232, 32, 170, 100, 135, 96, 211, 218, 217, 73, 56, 235, 20, 116, 214, 213, 69, 56, 235, 18, 116, 214, 221, 77, 56, 235, 22, 116, 214, 211, 67, 56, 235, 17, 116, 214, 219, 75, 56, 235, 21, 116, 214, 215, 71, 56, 235, 19, 116, 214, 223, 79, 56, 235, 23, 116, 54, 48, 64, 56, 27, 16, 116, 54, 56, 72, 56, 27, 20, 116, 54, 52, 68, 56, 27, 18, 116, 54, 60, 76, 56, 27, 86, 28, 156, 15, 1, 131, 243, 97, 162, 7, 184, 174, 56, 56, 31, 0, 6, 231, 131, 4, 157, 27, 138, 131, 243, 62, 96, 112, 222, 79, 208, 185, 169, 56, 56, 239, 1, 6, 231, 189, 4, 157, 91, 138, 131, 243, 46, 96, 112, 222, 77, 208, 185, 45, 57, 155, 185, 69, 136, 220, 38, 248, 220, 145, 228, 115, 131, 16, 185, 73, 240, 185, 171, 56, 56, 239, 36, 240, 92, 39, 240, 220, 147, 140, 207, 93, 66, 228, 30, 193, 103, 94, 145, 79, 1, 49, 75, 36, 16, 223, 151, 76, 207, 29, 66, 100, 158, 72, 207, 125, 193, 102, 169, 6, 25, 69, 219, 15, 4, 63, 0, 83, 139, 204, 162, 237, 135, 130, 95, 128, 169, 67, 134, 209, 246, 35, 193, 79, 192, 212, 35, 211, 104, 251, 177, 224, 55, 96, 26, 144, 113, 180, 253, 68, 240, 35, 48, 141, 200, 60, 218, 126, 42, 248, 21, 152, 38, 100, 32, 109, 63, 19, 252, 12, 76, 51, 50, 145, 182, 159, 171, 89, 51, 13, 163, 5, 25, 73, 219, 47, 212, 172, 185, 12, 163, 21, 153, 73, 219, 47, 213, 172, 121, 12, 163, 13, 25, 74, 219, 175, 212, 172, 89, 134, 209, 142, 76, 165, 237, 5, 201, 103, 236, 87, 132, 200, 2, 193, 231, 181, 36, 159, 23, 132, 200, 75, 130, 207, 27, 73, 62, 207, 8, 145, 231, 4, 159, 183, 146, 124, 158, 16, 34, 79, 9, 62, 239, 36, 249, 60, 34, 68, 30, 19, 124, 222, 75, 242, 121, 71, 136, 188, 39, 248, 124, 144, 228, 243, 134, 16, 121, 75, 240, 249, 40, 201, 231, 33, 33, 242, 154, 224, 243, 73, 146, 207, 71, 66, 228, 19, 193, 231, 179, 226, 145, 31, 219, 8, 60, 15, 8, 145, 47, 146, 241, 249, 64, 136, 124, 38, 226, 243, 229, 191, 122, 138, 228, 54, 224, 58, 74, 162, 243, 161, 174, 146, 237, 248, 84, 191, 100, 59, 209, 108, 238, 16, 252, 189, 162, 100, 71, 72, 100, 37, 241, 239, 192, 1, 184, 131, 193, 63, 1];
const bit_and_witnesses: Vec<u8> = vec![173, 213, 69, 110, 236, 64, 20, 70, 225, 199, 204, 204, 204, 232, 106, 219, 221, 246, 195, 126, 204, 76, 122, 33, 41, 173, 184, 3, 179, 48, 103, 7, 97, 152, 100, 20, 134, 121, 152, 25, 148, 13, 100, 146, 181, 68, 202, 22, 206, 95, 11, 248, 116, 234, 170, 116, 107, 109, 83, 243, 230, 213, 168, 5, 207, 22, 74, 24, 107, 43, 175, 216, 198, 137, 237, 252, 34, 59, 120, 197, 78, 78, 236, 226, 196, 110, 62, 139, 61, 188, 98, 47, 39, 246, 113, 98, 63, 39, 14, 112, 226, 32, 39, 14, 113, 226, 48, 39, 142, 112, 226, 40, 39, 142, 113, 226, 56, 39, 78, 112, 226, 36, 39, 78, 113, 226, 52, 39, 206, 240, 149, 115, 150, 87, 156, 227, 21, 231, 121, 197, 5, 78, 92, 228, 196, 37, 78, 92, 230, 196, 21, 78, 92, 229, 196, 53, 78, 92, 231, 196, 13, 254, 58, 111, 242, 138, 91, 188, 226, 54, 175, 184, 195, 137, 187, 156, 184, 199, 137, 251, 156, 120, 192, 137, 135, 156, 176, 56, 97, 56, 17, 226, 132, 205, 9, 135, 19, 46, 39, 194, 156, 136, 112, 194, 227, 132, 207, 137, 71, 156, 120, 204, 137, 39, 152, 48, 79, 121, 197, 51, 78, 60, 231, 68, 148, 19, 47, 56, 241, 146, 19, 175, 56, 241, 154, 19, 111, 56, 241, 150, 19, 239, 56, 241, 158, 19, 31, 56, 241, 145, 19, 159, 56, 241, 153, 19, 95, 56, 241, 149, 19, 223, 56, 241, 157, 239, 206, 31, 188, 226, 39, 39, 126, 113, 226, 55, 39, 254, 112, 226, 47, 39, 254, 113, 226, 63, 39, 18, 56, 145, 200, 137, 36, 78, 36, 115, 34, 133, 19, 1, 39, 226, 171, 81, 219, 10, 59, 78, 16, 9, 5, 198, 54, 169, 86, 200, 143, 121, 174, 229, 184, 177, 176, 103, 60, 227, 122, 110, 90, 200, 179, 237, 192, 115, 188, 136, 31, 243, 35, 150, 111, 28, 59, 48, 113, 215, 183, 131, 248, 198, 49, 233, 188, 34, 131, 87, 100, 242, 138, 44, 94, 145, 205, 43, 114, 56, 145, 203, 137, 60, 78, 228, 115, 162, 128, 19, 133, 156, 40, 226, 68, 49, 39, 74, 56, 81, 202, 137, 50, 78, 148, 115, 162, 130, 19, 43, 149, 2, 163, 74, 96, 84, 11, 140, 26, 129, 81, 43, 48, 234, 4, 70, 189, 192, 104, 16, 24, 141, 2, 163, 73, 96, 180, 8, 140, 86, 129, 209, 38, 48, 218, 249, 15, 187, 210, 33, 232, 232, 20, 116, 116, 9, 58, 186, 5, 70, 143, 192, 232, 21, 204, 163, 79, 208, 209, 47, 48, 6, 4, 198, 160, 192, 24, 18, 24, 195, 2, 99, 68, 96, 140, 10, 140, 49, 129, 49, 46, 48, 38, 4, 198, 164, 192, 152, 18, 24, 211, 2, 99, 70, 96, 204, 10, 140, 57, 129, 49, 47, 48, 22, 4, 198, 162, 192, 88, 18, 24, 203, 220, 88, 7];

#[cfg(test)]
mod test {
    use super::*;

    

    #[test]
    fn deserialize() {
        let _add_circuit = Circuit::read(&*add_circuit).unwrap();
        println!("add_circuit: {:?}", _add_circuit);
        let _add_witnesses: WitnessMap = WitnessMap::try_from(&add_witnesses[..]).unwrap();
        println!("add_witnesses: {:?}", _add_witnesses);

        let _bit_and_circuit = Circuit::read(&*bit_and_circuit).unwrap();
        println!("add_circuit: {:?}", _bit_and_circuit);
        let _bit_and_witnesses: WitnessMap = WitnessMap::try_from(&bit_and_witnesses[..]).unwrap();
        println!("add_witnesses: {:?}", _bit_and_witnesses);
    }

    fn test_add_circuit() {
        // let translator = NoirHalo2Translator::<Fr> {
        //     circuit: circuit.clone(),
        //     witness_values,
        //     _marker: PhantomData::<Fr>,
        // };
    }
}