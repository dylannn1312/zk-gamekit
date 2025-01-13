import { SudokuProvider } from "@/components/sudoku/context/SudokuContext";
import { SudokuGame } from "@/components/sudoku/Game";
import "./App.css"

export default function SudokuGamePage() {
    return (
        <SudokuProvider>
            <SudokuGame />
        </SudokuProvider>
    )
}
