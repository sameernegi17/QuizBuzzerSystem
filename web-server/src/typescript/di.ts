import { HttpService } from "./http.service.js"
import { QuestionService } from "./question.service.js";

const httpService = new HttpService();
const questionService = new QuestionService();

export default {
  httpService,
  questionService,
}