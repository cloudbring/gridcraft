import json,glob,math,sys
rows=[]
for f in sorted(glob.glob('cf-real-*.json')):
    for i in json.load(open(f))['inputs']:
        r=i['wall']['ratio']; rows.append((i['name'],r['point'],r['ci_lo'],r['ci_hi']))
n=len(rows)
lg=[-math.log(p) for _,p,_,_ in rows]   # speedup = old/new
se=[(math.log(h)-math.log(l))/3.92 for _,_,l,h in rows]
m=sum(lg)/n; s=math.sqrt(sum(x*x for x in se))/n
print("n",n,"pooled speedup %.3f [%.3f, %.3f]"%(math.exp(m),math.exp(m-1.96*s),math.exp(m+1.96*s)))
slower=[(x[0].split('/')[-1],1/x[1],1/x[3],1/x[2]) for x in rows if 1/x[2]<1]   # CI upper of speedup <1
faster=[x for x in rows if 1/x[3]>1]
print("slower (CI upper<1):",len(slower),"faster (CI lower>1):",len(faster),"inconclusive:",n-len(slower)-len(faster))
for x in sorted(slower,key=lambda x:x[1])[:15]: print("  %-50s %.2fx [%.2f, %.2f]"%x)
print("min point", min(1/x[1] for x in rows), "max", max(1/x[1] for x in rows))
# per-file absolute loss
for x in sorted(rows,key=lambda r:-r[1])[:5]: print(x[0].split('/')[-1],"%.2fx"%(1/x[1]))
