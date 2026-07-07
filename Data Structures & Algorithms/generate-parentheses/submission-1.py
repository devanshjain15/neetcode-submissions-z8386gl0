class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        result = []
        f(n, 0, 0, "", result)
        return result
        

def f(n: int, open_para: int, close_para: int, cur: str, result: List[str]):
    if open_para == close_para and open_para == n: 
        result.append(cur)
        return

    if close_para > open_para or open_para > n: 
        return

    f(n, open_para + 1, close_para, cur + "(", result)
    f(n, open_para, close_para + 1, cur + ")", result)