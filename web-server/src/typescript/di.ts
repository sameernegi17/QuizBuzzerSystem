import { BackendService } from "./backend.service.js";
import { HttpService } from "./http.service.js"
import { QuestionService } from "./question.service.js";

const httpService = new HttpService();
const questionService = new QuestionService();
const backendService = new BackendService(httpService);

export default {
  httpService,
  questionService,
  backendService,
}