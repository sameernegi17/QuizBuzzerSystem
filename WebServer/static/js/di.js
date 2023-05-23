import { HttpService } from "./http.service.js";
import { QuestionService } from "./question.service.js";
var httpService = new HttpService();
var questionService = new QuestionService();
export default {
    httpService: httpService,
    questionService: questionService,
};
