var QuestionService = /** @class */ (function () {
    function QuestionService() {
    }
    QuestionService.prototype.getRandomQuestion = function () {
        var randomNumber = Math.floor(Math.random() * 5);
        return QuestionService.QUESTIONS[randomNumber];
    };
    QuestionService.QUESTIONS = [
        "If a coding error falls in the forest, does it make a debugging sound?",
        "Can a computer code be ticklish?",
        "What if programming languages were replaced with different types of pasta?",
        "If a software bug had a superhero alter ego, what would its name be?",
        "What if computer keyboards had a built-in sarcasm button?"
    ];
    return QuestionService;
}());
export { QuestionService };
